"""Test configuration and fixtures."""

import pytest
from pathlib import Path


@pytest.fixture
def fixtures_path() -> Path:
    """Return path to test fixtures directory."""
    return Path(__file__).parent / "fixtures"


@pytest.fixture
def sample_demo_data() -> dict:
    """Return sample parsed demo data for testing."""
    return {
        "metadata": {
            "map_name": "de_dust2",
            "played_at": "2024-01-15T14:30:00Z",
            "duration_ticks": 256000,
            "tickrate": 64,
        },
        "players": [
            {"steam_id": "76561198012345678", "name": "Player1", "team": "CT"},
            {"steam_id": "76561198087654321", "name": "Player2", "team": "T"},
        ],
        "rounds": [
            {"number": 1, "winner": "CT", "start_tick": 1000, "end_tick": 5000},
            {"number": 2, "winner": "T", "start_tick": 5500, "end_tick": 10000},
        ],
        "events": [
            {
                "tick": 1500,
                "round": 1,
                "type": "KILL",
                "data": {
                    "attacker_id": "76561198012345678",
                    "victim_id": "76561198087654321",
                    "weapon": "ak47",
                    "headshot": True,
                },
            },
        ],
        "stats": {
            "players": {
                "76561198012345678": {"kills": 1, "deaths": 0, "assists": 0},
                "76561198087654321": {"kills": 0, "deaths": 1, "assists": 0},
            }
        },
    }
