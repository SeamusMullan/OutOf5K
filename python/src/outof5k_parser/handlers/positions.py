"""Handler for the get_positions command."""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Any

from ..parsing.parser import DemoParser

if TYPE_CHECKING:
    from collections.abc import Callable


def handle_get_positions(
    params: dict[str, Any], progress: Callable[[int, str], None]
) -> dict[str, Any]:
    """Get player positions for a tick range.

    Args:
        params: Command parameters containing:
            - file_path: Path to the demo file
            - start_tick: Start tick for position extraction
            - end_tick: End tick for position extraction
            - interval: Tick interval for sampling (default: 8)
        progress: Callback for progress updates

    Returns:
        Position data for all players in the tick range.

    Raises:
        FileNotFoundError: If the demo file doesn't exist.
        ValueError: If required parameters are missing.
    """
    file_path = params.get("file_path")
    if not file_path:
        raise ValueError("Missing required parameter: file_path")

    path = Path(file_path)
    if not path.exists():
        raise FileNotFoundError(f"Demo file not found: {file_path}")

    start_tick = params.get("start_tick", 0)
    end_tick = params.get("end_tick")
    interval = params.get("interval", 8)

    parser = DemoParser(path)
    return parser.get_positions(
        start_tick=start_tick,
        end_tick=end_tick,
        interval=interval,
        progress_callback=progress,
    )
