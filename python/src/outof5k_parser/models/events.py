"""Event-related data models."""

from __future__ import annotations

from typing import Any, Literal

from pydantic import BaseModel


class GameEvent(BaseModel):
    """Base class for game events."""

    tick: int
    round: int
    type: str
    data: dict[str, Any] = {}


class KillEvent(BaseModel):
    """A kill event in the demo."""

    tick: int
    round: int
    attacker_id: str
    victim_id: str
    weapon: str
    headshot: bool = False
    wallbang: bool = False
    through_smoke: bool = False
    no_scope: bool = False
    attacker_blind: bool = False
    assister_id: str | None = None


class GrenadeEvent(BaseModel):
    """A grenade-related event."""

    tick: int
    round: int
    thrower_id: str
    grenade_type: Literal["smoke", "flashbang", "he_grenade", "molotov", "incendiary", "decoy"]
    x: float
    y: float
    z: float


class PositionSnapshot(BaseModel):
    """Position data for a single player at a tick."""

    x: float
    y: float
    z: float
    view_x: float  # Yaw
    view_y: float  # Pitch
    health: int
    armor: int


class TickPositions(BaseModel):
    """Position data for all players at a specific tick."""

    tick: int
    players: dict[str, PositionSnapshot]
