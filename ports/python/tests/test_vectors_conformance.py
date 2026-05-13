"""Conformance test: load each test vector and verify the Python port
reproduces it exactly. Every other port (Ruby, Rust, Go, ...) will run
this same logic against the same vectors.
"""

import json
from pathlib import Path

import pytest

from naipes.cli.loop import replay


VECTORS_DIR = Path(__file__).parent.parent.parent.parent / "spec" / "vectors"


def _all_vectors():
    return sorted(VECTORS_DIR.glob("v*.json"))


@pytest.mark.parametrize("vector_path", _all_vectors(), ids=lambda p: p.stem)
def test_vector_conformance(vector_path: Path):
    with open(vector_path, encoding="utf-8") as f:
        vector = json.load(f)

    result = replay(
        seed=vector["seed"],
        ai_level=vector["ai_level"],
        human_plays=vector["human_plays"],
    )

    # Final result must match exactly.
    assert result.human_score == vector["expected_result"]["human"], (
        f"{vector['id']}: human score mismatch"
    )
    assert result.ai_score == vector["expected_result"]["ai"], (
        f"{vector['id']}: ai score mismatch"
    )
    assert result.outcome == vector["expected_result"]["outcome"], (
        f"{vector['id']}: outcome mismatch"
    )

    # Trick log must match exactly, card by card.
    assert len(result.trick_log) == len(vector["expected_trick_log"]), (
        f"{vector['id']}: trick count mismatch"
    )
    for i, (got, expected) in enumerate(
        zip(result.trick_log, vector["expected_trick_log"])
    ):
        assert got.trick_number == expected["trick"], f"{vector['id']} trick {i+1}"
        assert got.leader.value == expected["leader"], f"{vector['id']} trick {i+1} leader"
        assert got.human_card.label == expected["human_card"], (
            f"{vector['id']} trick {i+1} human_card"
        )
        assert got.ai_card.label == expected["ai_card"], (
            f"{vector['id']} trick {i+1} ai_card"
        )
        assert got.winner.value == expected["winner"], (
            f"{vector['id']} trick {i+1} winner"
        )
        assert got.points == expected["points"], f"{vector['id']} trick {i+1} points"
