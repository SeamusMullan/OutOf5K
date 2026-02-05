"""Handler for the get_metadata command."""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Any

from ..parsing.parser import DemoParser

if TYPE_CHECKING:
    from collections.abc import Callable


def handle_get_metadata(
    params: dict[str, Any], _progress: Callable[[int, str], None]
) -> dict[str, Any]:
    """Get demo metadata without full parsing.

    Args:
        params: Command parameters containing:
            - file_path: Path to the demo file
        _progress: Progress callback (unused for metadata extraction)

    Returns:
        Demo metadata including map name, played_at, duration, tickrate.

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

    parser = DemoParser(path)
    return parser.get_metadata()
