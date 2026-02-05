"""Statistics calculation algorithms."""

from __future__ import annotations

from typing import Any


def calculate_stats(demo_data: dict[str, Any]) -> dict[str, Any]:
    """Calculate comprehensive statistics from demo data.

    Args:
        demo_data: Parsed demo data with events and players.

    Returns:
        Dictionary containing player stats, team stats, and utility stats.
    """
    events = demo_data.get("events", [])
    players = demo_data.get("players", [])
    rounds = demo_data.get("rounds", [])

    # Initialize player stats
    player_stats: dict[str, dict[str, Any]] = {}
    for player in players:
        steam_id = player.get("steam_id", "")
        player_stats[steam_id] = {
            "kills": 0,
            "deaths": 0,
            "assists": 0,
            "headshots": 0,
            "damage": 0,
            "adr": 0.0,
            "kd_ratio": 0.0,
            "headshot_percentage": 0.0,
            "first_kills": 0,
            "first_deaths": 0,
        }

    # Track first kills per round
    first_kill_rounds: set[int] = set()

    # Process events
    for event in events:
        if event.get("type") == "KILL":
            data = event.get("data", {})
            round_num = event.get("round", 0)
            attacker_id = data.get("attacker_id", "")
            victim_id = data.get("victim_id", "")

            # Count kills
            if attacker_id in player_stats:
                player_stats[attacker_id]["kills"] += 1
                if data.get("headshot"):
                    player_stats[attacker_id]["headshots"] += 1

                # First kill tracking
                if round_num not in first_kill_rounds:
                    first_kill_rounds.add(round_num)
                    player_stats[attacker_id]["first_kills"] += 1
                    if victim_id in player_stats:
                        player_stats[victim_id]["first_deaths"] += 1

            # Count deaths
            if victim_id in player_stats:
                player_stats[victim_id]["deaths"] += 1

            # Count assists
            assister_id = data.get("assister_id")
            if assister_id and assister_id in player_stats:
                player_stats[assister_id]["assists"] += 1

    # Calculate derived stats
    total_rounds = len(rounds) if rounds else 1

    for steam_id, stats in player_stats.items():
        kills = stats["kills"]
        deaths = stats["deaths"]
        headshots = stats["headshots"]

        # K/D ratio
        stats["kd_ratio"] = round(kills / max(deaths, 1), 2)

        # Headshot percentage
        stats["headshot_percentage"] = round((headshots / max(kills, 1)) * 100, 1)

        # ADR placeholder (would need damage events for accurate calculation)
        stats["adr"] = round(stats["damage"] / total_rounds, 1) if stats["damage"] > 0 else 0.0

    # Utility stats placeholder
    utility_stats = {
        "grenades_thrown": 0,
        "damage_dealt": 0,
        "enemies_flashed": 0,
        "flash_duration_avg": 0.0,
    }

    return {
        "players": player_stats,
        "utility": utility_stats,
        "rounds_played": total_rounds,
    }
