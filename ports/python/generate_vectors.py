"""Generate canonical test vectors that all naipes ports must pass.

Usage:
    python generate_vectors.py spec/vectors/

Produces v001.json .. v020.json. Each vector fixes:
- seed: u64
- ai_level: easy | normal
- human_plays: list of 1-indexed hand positions (always 1 here for simplicity,
  could be made strategic in future revisions)
- expected_result: final scores and outcome
- expected_trick_log: card-by-card play sequence

For determinism, the human always plays index 1 in every vector. This makes
the vector compact and unambiguous: the entire game is a function of seed
and AI level.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from naipes.cli.loop import replay
from naipes.games.brisca.state import Player


def vector_for(vector_id: str, seed: int, ai_level: str) -> dict:
    human_plays = [1] * 20  # always play first card in hand
    result = replay(seed=seed, ai_level=ai_level, human_plays=human_plays)
    trick_log = [
        {
            "trick": r.trick_number,
            "leader": r.leader.value,
            "human_card": r.human_card.label,
            "ai_card": r.ai_card.label,
            "winner": r.winner.value,
            "points": r.points,
        }
        for r in result.trick_log
    ]
    return {
        "id": vector_id,
        "seed": seed,
        "ai_level": ai_level,
        "human_plays": human_plays,
        "expected_result": {
            "human": result.human_score,
            "ai": result.ai_score,
            "outcome": result.outcome,
        },
        "expected_trick_log": trick_log,
    }


def main(out_dir: Path) -> int:
    out_dir.mkdir(parents=True, exist_ok=True)
    # 20 vectors: 10 with easy AI, 10 with normal AI, seeds 1..10 each.
    vectors = []
    for i, seed in enumerate(range(1, 11), start=1):
        v = vector_for(f"v{i:03d}", seed, "easy")
        vectors.append(v)
    for i, seed in enumerate(range(1, 11), start=11):
        v = vector_for(f"v{i:03d}", seed, "normal")
        vectors.append(v)

    for v in vectors:
        path = out_dir / f"{v['id']}.json"
        with open(path, "w", encoding="utf-8") as f:
            json.dump(v, f, indent=2, ensure_ascii=False)
        print(f"Wrote {path}: seed={v['seed']} ai={v['ai_level']} "
              f"-> human={v['expected_result']['human']} "
              f"ai={v['expected_result']['ai']} "
              f"outcome={v['expected_result']['outcome']}")
    return 0


if __name__ == "__main__":
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("vectors")
    sys.exit(main(out))
