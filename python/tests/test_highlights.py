"""Tests for highlight detection."""

from outof5k_parser.analysis.highlights import find_highlights


def test_find_ace(sample_demo_data):
    """Test detecting an ace (5 kills in a round)."""
    # Create 5 kills in round 1
    sample_demo_data["events"] = [
        {
            "tick": 1000 + i * 100,
            "round": 1,
            "type": "KILL",
            "data": {"attacker_id": "player1", "victim_id": f"victim{i}"},
        }
        for i in range(5)
    ]

    highlights = find_highlights(sample_demo_data)

    assert len(highlights) == 1
    assert highlights[0]["type"] == "ACE"
    assert highlights[0]["kills"] == 5


def test_find_triple_kill(sample_demo_data):
    """Test detecting a triple kill."""
    sample_demo_data["events"] = [
        {
            "tick": 1000 + i * 100,
            "round": 1,
            "type": "KILL",
            "data": {"attacker_id": "player1", "victim_id": f"victim{i}"},
        }
        for i in range(3)
    ]

    highlights = find_highlights(sample_demo_data)

    assert len(highlights) == 1
    assert highlights[0]["type"] == "TRIPLE_KILL"
    assert highlights[0]["kills"] == 3


def test_no_highlights_for_low_kills(sample_demo_data):
    """Test that fewer than 3 kills don't create highlights."""
    sample_demo_data["events"] = [
        {
            "tick": 1000,
            "round": 1,
            "type": "KILL",
            "data": {"attacker_id": "player1", "victim_id": "victim1"},
        },
        {
            "tick": 1100,
            "round": 1,
            "type": "KILL",
            "data": {"attacker_id": "player1", "victim_id": "victim2"},
        },
    ]

    highlights = find_highlights(sample_demo_data)

    assert len(highlights) == 0
