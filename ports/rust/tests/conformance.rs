//! Conformance test: load each test vector from spec/vectors/ and verify
//! the Rust port reproduces it exactly, baza por baza.
//!
//! If this test fails, the Rust port has diverged from spec.

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use naipes::brisca::ai::AiLevel;
use naipes::cli::replay;

#[derive(Debug, Deserialize)]
struct ExpectedResult {
    human: u32,
    ai: u32,
    outcome: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedTrick {
    trick: u8,
    leader: String,
    human_card: String,
    ai_card: String,
    winner: String,
    points: u8,
}

#[derive(Debug, Deserialize)]
struct Vector {
    id: String,
    seed: u64,
    ai_level: String,
    human_plays: Vec<u8>,
    expected_result: ExpectedResult,
    expected_trick_log: Vec<ExpectedTrick>,
}

fn vectors_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .join("..")
        .join("..")
        .join("spec")
        .join("vectors")
}

#[test]
fn all_vectors_pass() {
    let dir = vectors_dir();
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {dir:?}: {e}"))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.starts_with('v'))
                .unwrap_or(false)
        })
        .collect();
    paths.sort();

    assert!(!paths.is_empty(), "no vector files found in {dir:?}");

    let mut failures = Vec::new();

    for path in &paths {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("cannot read {path:?}: {e}"));
        let vector: Vector = serde_json::from_str(&content)
            .unwrap_or_else(|e| panic!("cannot parse {path:?}: {e}"));

        let ai_level = AiLevel::parse(&vector.ai_level)
            .unwrap_or_else(|| panic!("unknown ai_level in {}: {}", vector.id, vector.ai_level));

        let result = replay(vector.seed, ai_level, &vector.human_plays);

        let mut errors = Vec::new();

        if result.human_score != vector.expected_result.human {
            errors.push(format!(
                "human score: got {}, expected {}",
                result.human_score, vector.expected_result.human
            ));
        }
        if result.ai_score != vector.expected_result.ai {
            errors.push(format!(
                "ai score: got {}, expected {}",
                result.ai_score, vector.expected_result.ai
            ));
        }
        if result.outcome.as_str() != vector.expected_result.outcome {
            errors.push(format!(
                "outcome: got {}, expected {}",
                result.outcome.as_str(),
                vector.expected_result.outcome
            ));
        }
        if result.trick_log.len() != vector.expected_trick_log.len() {
            errors.push(format!(
                "trick count: got {}, expected {}",
                result.trick_log.len(),
                vector.expected_trick_log.len()
            ));
        }

        for (i, (got, expected)) in result
            .trick_log
            .iter()
            .zip(vector.expected_trick_log.iter())
            .enumerate()
        {
            if got.trick_number != expected.trick {
                errors.push(format!("trick {} number mismatch", i + 1));
            }
            if got.leader.as_str() != expected.leader {
                errors.push(format!(
                    "trick {} leader: got {}, expected {}",
                    i + 1,
                    got.leader.as_str(),
                    expected.leader
                ));
            }
            if got.human_card.label() != expected.human_card {
                errors.push(format!(
                    "trick {} human_card: got {}, expected {}",
                    i + 1,
                    got.human_card.label(),
                    expected.human_card
                ));
            }
            if got.ai_card.label() != expected.ai_card {
                errors.push(format!(
                    "trick {} ai_card: got {}, expected {}",
                    i + 1,
                    got.ai_card.label(),
                    expected.ai_card
                ));
            }
            if got.winner.as_str() != expected.winner {
                errors.push(format!(
                    "trick {} winner: got {}, expected {}",
                    i + 1,
                    got.winner.as_str(),
                    expected.winner
                ));
            }
            if got.points != expected.points {
                errors.push(format!(
                    "trick {} points: got {}, expected {}",
                    i + 1,
                    got.points,
                    expected.points
                ));
            }
        }

        if !errors.is_empty() {
            failures.push(format!(
                "{}: {} error(s):\n  - {}",
                vector.id,
                errors.len(),
                errors.join("\n  - ")
            ));
        }
    }

    if !failures.is_empty() {
        panic!(
            "{}/{} vector(s) failed conformance:\n\n{}",
            failures.len(),
            paths.len(),
            failures.join("\n\n")
        );
    }
}
