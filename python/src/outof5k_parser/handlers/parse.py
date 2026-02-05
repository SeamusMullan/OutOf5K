"""Handler for the parse command."""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Any

from ..parsing.parser import DemoParser

if TYPE_CHECKING:
    from collections.abc import Callable


def handle_parse(params: dict[str, Any], progress: Callable[[int, str], None]) -> dict[str, Any]:
    """Parse a demo file and extract all data.

    Args:
        params: Command parameters containing:
            - file_path: Path to the demo file
            - options: Optional parsing options
                - extract_positions: Whether to extract position data (default: True)
                - position_interval: Tick interval for position sampling (default: 16)
                - extract_voice: Whether to extract voice data (default: False)
        progress: Callback for progress updates

    Returns:
        Parsed demo data including metadata, players, rounds, events, and stats.

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

    options = params.get("options", {})

    parser = DemoParser(path)
    return parser.parse(
        extract_positions=options.get("extract_positions", True),
        position_interval=options.get("position_interval", 16),
        extract_voice=options.get("extract_voice", False),
        progress_callback=progress,
    )
