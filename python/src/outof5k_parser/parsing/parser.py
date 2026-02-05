"""Main demo parser using demoparser2."""

from __future__ import annotations

from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import Callable


class DemoParser:
    """Parser for CS2 demo files using demoparser2.

    This class wraps demoparser2 to extract data from CS2 .dem files.
    """

    def __init__(self, file_path: Path) -> None:
        """Initialize the parser with a demo file path.

        Args:
            file_path: Path to the .dem file to parse.
        """
        self.file_path = file_path
        self._parser: Any = None

    def _get_parser(self) -> Any:
        """Lazily initialize and return the demoparser2 parser."""
        if self._parser is None:
            from demoparser2 import DemoParser as DP2

            self._parser = DP2(str(self.file_path))
        return self._parser

    def get_metadata(self) -> dict[str, Any]:
        """Extract just the metadata from the demo without full parsing.

        Returns:
            Dictionary containing map_name, played_at, duration_ticks, tickrate.
        """
        parser = self._get_parser()
        header = parser.parse_header()

        return {
            "map_name": header.get("map_name", "unknown"),
            "played_at": self._extract_played_at(header),
            "duration_ticks": header.get("playback_ticks", 0),
            "tickrate": header.get("playback_tick_rate", 64),
            "server_name": header.get("server_name"),
        }

    def parse(
        self,
        extract_positions: bool = True,
        position_interval: int = 16,
        extract_voice: bool = False,
        progress_callback: Callable[[int, str], None] | None = None,
    ) -> dict[str, Any]:
        """Parse the demo file and extract all data.

        Args:
            extract_positions: Whether to extract player position data.
            position_interval: Tick interval for position sampling.
            extract_voice: Whether to extract voice data (not implemented).
            progress_callback: Callback for progress updates.

        Returns:
            Complete parsed demo data including metadata, players, rounds, events, stats.
        """
        if progress_callback:
            progress_callback(5, "Extracting header...")

        metadata = self.get_metadata()

        if progress_callback:
            progress_callback(10, "Parsing players...")

        players = self._extract_players()

        if progress_callback:
            progress_callback(20, "Parsing rounds...")

        rounds = self._extract_rounds()

        if progress_callback:
            progress_callback(40, "Parsing events...")

        events = self._extract_events()

        if progress_callback:
            progress_callback(60, "Calculating statistics...")

        stats = self._calculate_stats(events, players)

        result: dict[str, Any] = {
            "metadata": metadata,
            "players": players,
            "rounds": rounds,
            "events": events,
            "stats": stats,
        }

        if extract_positions:
            if progress_callback:
                progress_callback(80, "Extracting positions...")
            # Position extraction is expensive, so we make it optional
            result["positions"] = self._extract_all_positions(position_interval)

        if progress_callback:
            progress_callback(100, "Parsing complete")

        return result

    def get_positions(
        self,
        start_tick: int = 0,
        end_tick: int | None = None,
        interval: int = 8,
        progress_callback: Callable[[int, str], None] | None = None,
    ) -> dict[str, Any]:
        """Get player positions for a specific tick range.

        Args:
            start_tick: Starting tick.
            end_tick: Ending tick (None for end of demo).
            interval: Tick interval for sampling.
            progress_callback: Callback for progress updates.

        Returns:
            Dictionary with positions array.
        """
        if progress_callback:
            progress_callback(10, "Extracting positions...")

        parser = self._get_parser()

        # Properties to extract for positions
        props = [
            "X",
            "Y",
            "Z",
            "pitch",
            "yaw",
            "health",
            "armor_value",
            "steamid",
        ]

        try:
            # Get tick data
            ticks_df = parser.parse_ticks(props)

            # Filter tick range
            if end_tick is not None:
                ticks_df = ticks_df[
                    (ticks_df["tick"] >= start_tick) & (ticks_df["tick"] <= end_tick)
                ]
            else:
                ticks_df = ticks_df[ticks_df["tick"] >= start_tick]

            # Sample at interval
            unique_ticks = sorted(ticks_df["tick"].unique())
            sampled_ticks = unique_ticks[::interval]

            if progress_callback:
                progress_callback(50, "Processing position data...")

            positions = []
            for tick in sampled_ticks:
                tick_data = ticks_df[ticks_df["tick"] == tick]
                players_data: dict[str, dict[str, Any]] = {}

                for _, row in tick_data.iterrows():
                    steam_id = str(row.get("steamid", ""))
                    if steam_id:
                        players_data[steam_id] = {
                            "x": float(row.get("X", 0)),
                            "y": float(row.get("Y", 0)),
                            "z": float(row.get("Z", 0)),
                            "view_x": float(row.get("yaw", 0)),
                            "view_y": float(row.get("pitch", 0)),
                            "health": int(row.get("health", 0)),
                            "armor": int(row.get("armor_value", 0)),
                        }

                positions.append({"tick": tick, "players": players_data})

            if progress_callback:
                progress_callback(100, "Position extraction complete")

            return {"positions": positions}

        except Exception as e:
            # Return empty positions on error
            if progress_callback:
                progress_callback(100, f"Position extraction failed: {e}")
            return {"positions": []}

    def _extract_played_at(self, header: dict[str, Any]) -> str | None:
        """Extract the played_at timestamp from header."""
        # demoparser2 may provide this in different formats
        if "game_start_time" in header:
            try:
                ts = int(header["game_start_time"])
                return datetime.fromtimestamp(ts, tz=timezone.utc).isoformat()
            except (ValueError, TypeError):
                pass
        return None

    def _extract_players(self) -> list[dict[str, Any]]:
        """Extract player information from the demo."""
        parser = self._get_parser()

        try:
            # Get player info from the demo
            players_df = parser.parse_player_info()
            players = []

            for _, row in players_df.iterrows():
                steam_id = str(row.get("steamid", ""))
                if steam_id and steam_id != "0":
                    players.append(
                        {
                            "steam_id": steam_id,
                            "name": str(row.get("name", "Unknown")),
                            "team": self._get_team_name(row.get("team_num", 0)),
                        }
                    )

            return players
        except Exception:
            return []

    def _extract_rounds(self) -> list[dict[str, Any]]:
        """Extract round information from the demo."""
        parser = self._get_parser()

        try:
            # Parse round events
            events = parser.parse_events(["round_start", "round_end"])
            rounds = []
            round_num = 0

            round_starts = events.get("round_start", [])
            round_ends = events.get("round_end", [])

            for i, start in enumerate(round_starts):
                round_num += 1
                end = round_ends[i] if i < len(round_ends) else {}

                rounds.append(
                    {
                        "number": round_num,
                        "winner": self._get_winner_from_event(end),
                        "win_reason": end.get("reason"),
                        "start_tick": start.get("tick", 0),
                        "end_tick": end.get("tick", 0),
                    }
                )

            return rounds
        except Exception:
            return []

    def _extract_events(self) -> list[dict[str, Any]]:
        """Extract game events (kills, etc.) from the demo."""
        parser = self._get_parser()

        try:
            # Parse kill events
            events = parser.parse_events(["player_death"])
            kill_events = events.get("player_death", [])

            result = []
            current_round = 1

            for event in kill_events:
                result.append(
                    {
                        "tick": event.get("tick", 0),
                        "round": current_round,
                        "type": "KILL",
                        "data": {
                            "attacker_id": str(event.get("attacker_steamid", "")),
                            "victim_id": str(event.get("user_steamid", "")),
                            "weapon": event.get("weapon", ""),
                            "headshot": event.get("headshot", False),
                        },
                    }
                )

            return result
        except Exception:
            return []

    def _calculate_stats(
        self, events: list[dict[str, Any]], players: list[dict[str, Any]]
    ) -> dict[str, Any]:
        """Calculate player statistics from events."""
        stats: dict[str, dict[str, int]] = {}

        # Initialize stats for all players
        for player in players:
            steam_id = player["steam_id"]
            stats[steam_id] = {
                "kills": 0,
                "deaths": 0,
                "assists": 0,
                "headshots": 0,
            }

        # Count kills and deaths
        for event in events:
            if event["type"] == "KILL":
                data = event["data"]
                attacker_id = data.get("attacker_id", "")
                victim_id = data.get("victim_id", "")

                if attacker_id in stats:
                    stats[attacker_id]["kills"] += 1
                    if data.get("headshot"):
                        stats[attacker_id]["headshots"] += 1

                if victim_id in stats:
                    stats[victim_id]["deaths"] += 1

        return {"players": stats}

    def _extract_all_positions(self, interval: int = 16) -> list[dict[str, Any]]:
        """Extract all position data from the demo."""
        result = self.get_positions(interval=interval)
        return result.get("positions", [])

    def _get_team_name(self, team_num: int) -> str:
        """Convert team number to team name."""
        if team_num == 2:
            return "T"
        elif team_num == 3:
            return "CT"
        return "UNASSIGNED"

    def _get_winner_from_event(self, event: dict[str, Any]) -> str | None:
        """Get the winning team from a round_end event."""
        winner = event.get("winner")
        if winner == 2:
            return "T"
        elif winner == 3:
            return "CT"
        return None
