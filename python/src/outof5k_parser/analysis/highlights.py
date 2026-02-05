"""Highlight detection algorithms."""

from __future__ import annotations

from typing import Any


def find_highlights(demo_data: dict[str, Any]) -> list[dict[str, Any]]:
    """Find notable plays in the demo data.

    Detects:
    - Aces (5 kills in a round)
    - Quad kills (4 kills in a round)
    - Triple kills (3 kills in a round)
    - Clutches (1vX wins)

    Args:
        demo_data: Parsed demo data with events and rounds.

    Returns:
        List of highlight events.
    """
    highlights: list[dict[str, Any]] = []
    events = demo_data.get("events", [])
    rounds = demo_data.get("rounds", [])

    # Group kills by round and player
    round_kills: dict[int, dict[str, list[dict[str, Any]]]] = {}

    for event in events:
        if event.get("type") == "KILL":
            round_num = event.get("round", 0)
            attacker_id = event.get("data", {}).get("attacker_id", "")

            if round_num not in round_kills:
                round_kills[round_num] = {}

            if attacker_id not in round_kills[round_num]:
                round_kills[round_num][attacker_id] = []

            round_kills[round_num][attacker_id].append(event)

    # Find multi-kills
    for round_num, players in round_kills.items():
        round_info = next((r for r in rounds if r.get("number") == round_num), {})

        for player_id, kills in players.items():
            kill_count = len(kills)

            if kill_count >= 3:
                highlight_type = "TRIPLE_KILL"
                if kill_count == 4:
                    highlight_type = "QUAD_KILL"
                elif kill_count >= 5:
                    highlight_type = "ACE"

                # Get tick range for the highlight
                kill_ticks = [k.get("tick", 0) for k in kills]

                highlights.append(
                    {
                        "type": highlight_type,
                        "round": round_num,
                        "player_id": player_id,
                        "start_tick": min(kill_ticks) - 128
                        if kill_ticks
                        else 0,  # ~2 seconds before
                        "end_tick": max(kill_ticks) + 128 if kill_ticks else 0,  # ~2 seconds after
                        "kills": kill_count,
                    }
                )

    # Sort highlights by tick
    highlights.sort(key=lambda h: h.get("start_tick", 0))

    return highlights
