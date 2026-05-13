/**
 * Brisca interactive loop and headless replay driver.
 *
 * `replay()` is the headless engine used by test vectors — no I/O.
 * `playInteractive()` is the stdin/stdout CLI entry point.
 */

import * as readline from "node:readline";
import { stdin, stdout } from "node:process";

import { choose as chooseAiMove, type AiLevel } from "../games/brisca/ai.js";
import {
  GameState,
  type Player,
  type TrickRecord,
} from "../games/brisca/state.js";
import { renderBoard, renderFinal, renderTrickResult } from "./render.js";

export interface ReplayResult {
  humanScore: number;
  aiScore: number;
  outcome: "win" | "loss" | "draw";
  trickLog: TrickRecord[];
}

function outcomeFor(human: number): "win" | "loss" | "draw" {
  if (human > 60) return "win";
  if (human < 60) return "loss";
  return "draw";
}

/**
 * Run a brisca game from a scripted sequence of human plays.
 *
 * `humanPlays` is a list of 1-indexed hand positions the human picks.
 */
export function replay(
  seed: bigint | number,
  aiLevel: AiLevel,
  humanPlays: readonly number[],
): ReplayResult {
  const state = GameState.new(seed);
  let playIdx = 0;

  while (!state.isFinished()) {
    if (state.leader === "human") {
      const idx = humanPlays[playIdx++]! - 1;
      state.playCard("human", idx);
      const aiIdx = chooseAiMove(state, aiLevel);
      state.playCard("ai", aiIdx);
    } else {
      const aiIdx = chooseAiMove(state, aiLevel);
      state.playCard("ai", aiIdx);
      const idx = humanPlays[playIdx++]! - 1;
      state.playCard("human", idx);
    }
  }

  return {
    humanScore: state.scoreOf("human"),
    aiScore: state.scoreOf("ai"),
    outcome: outcomeFor(state.scoreOf("human")),
    trickLog: state.trickLog,
  };
}

/** Run a brisca game with stdin/stdout. Returns process exit code. */
export async function playInteractive(
  seed: bigint | number,
  aiLevel: AiLevel,
  options: { quiet?: boolean } = {},
): Promise<number> {
  const quiet = options.quiet ?? false;
  const state = GameState.new(seed);

  const emit = (text = "") => stdout.write(text + "\n");
  const write = (text: string) => stdout.write(text);

  // Robust async line iterator over stdin. Works whether stdin is a TTY
  // or piped (scripted via `echo "1" | naipes play ...`).
  const rl = readline.createInterface({ input: stdin, crlfDelay: Infinity });
  const lines = rl[Symbol.asyncIterator]() as AsyncIterator<string>;

  const readLine = async (prompt: string): Promise<string | null> => {
    write(prompt);
    const next = await lines.next();
    if (next.done) return null;
    return next.value;
  };

  const promptHuman = async (): Promise<number | null> => {
    while (true) {
      emit(renderBoard(state));
      const raw = await readLine("> ");
      if (raw === null) return null;
      const norm = raw.trim().toLowerCase();
      if (norm === "q" || norm === "quit") return null;
      if (norm === "?" || norm === "help") {
        emit("Commands: 1-3 (play card), q (quit), ? (help)");
        continue;
      }
      if (norm === "1" || norm === "2" || norm === "3") {
        const idx1 = parseInt(norm, 10);
        if (idx1 > state.humanHand.length) {
          emit(`You only have ${state.humanHand.length} cards.`);
          continue;
        }
        return idx1;
      }
      emit("Invalid input. Use 1-3, q, or ?.");
    }
  };

  try {
    while (!state.isFinished()) {
      let humanIdx1: number | null;
      if (state.leader === "human") {
        humanIdx1 = await promptHuman();
        if (humanIdx1 === null) {
          emit("Game abandoned.");
          return 0;
        }
        state.playCard("human", humanIdx1 - 1);
        const aiIdx = chooseAiMove(state, aiLevel);
        state.playCard("ai", aiIdx);
      } else {
        const aiIdx = chooseAiMove(state, aiLevel);
        state.playCard("ai", aiIdx);
        humanIdx1 = await promptHuman();
        if (humanIdx1 === null) {
          emit("Game abandoned.");
          return 0;
        }
        state.playCard("human", humanIdx1 - 1);
      }

      emit(renderTrickResult(state));
      if (!quiet && !state.isFinished()) {
        await readLine("(Press Enter to continue) ");
      }
    }

    emit(renderFinal(state));
    return 0;
  } finally {
    rl.close();
  }
}
