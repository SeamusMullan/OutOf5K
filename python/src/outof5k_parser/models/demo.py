"""Demo-related data models."""

from __future__ import annotations

from datetime import datetime
from typing import Literal

from pydantic import BaseModel


class DemoMetadata(BaseModel):
    """Metadata extracted from a demo file header."""

    map_name: str
    played_at: datetime | None = None
    duration_ticks: int
    tickrate: int
    server_name: str | None = None


class PlayerInfo(BaseModel):
    """Information about a player in the demo."""

    steam_id: str
    name: str
    team: Literal["CT", "T", "UNASSIGNED"] = "UNASSIGNED"


class RoundInfo(BaseModel):
    """Information about a single round."""

    number: int
    winner: Literal["CT", "T"] | None = None
    win_reason: str | None = None
    start_tick: int
    end_tick: int
    ct_score: int = 0
    t_score: int = 0
