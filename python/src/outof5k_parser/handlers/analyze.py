"""Handler for the analyze command."""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Any

from ..analysis.highlights import find_highlights
from ..analysis.stats import calculate_stats
from ..parsing.parser import DemoParser

if TYPE_CHECKING:
    from collections.abc import Callable


def handle_analyze(params: dict[str, Any], progress: Callable[[int, str], None]) -> dict[str, Any]:
    """Run analysis on a demo file.

    Args:
        params: Command parameters containing:
            - file_path: Path to the demo file
            - analyzers: List of analyzers to run (default: all)
                - "highlights": Find notable plays (aces, clutches, etc.)
                - "utility": Analyze grenade usage
                - "positions": Analyze positioning patterns
        progress: Callback for progress updates

    Returns:
        Analysis results from requested analyzers.

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

    analyzers = params.get("analyzers", ["highlights", "utility", "positions"])

    progress(10, "Parsing demo file...")
    parser = DemoParser(path)
    demo_data = parser.parse(progress_callback=lambda p, m: progress(10 + p // 2, m))

    results: dict[str, Any] = {}
    analyzer_count = len(analyzers)

    for i, analyzer in enumerate(analyzers):
        base_progress = 60 + (i * 40) // analyzer_count
        progress(base_progress, f"Running {analyzer} analysis...")

        if analyzer == "highlights":
            results["highlights"] = find_highlights(demo_data)
        elif analyzer == "utility":
            results["utility"] = calculate_stats(demo_data).get("utility", {})
        elif analyzer == "positions":
            results["positions"] = demo_data.get("positions", {})

    progress(100, "Analysis complete")
    return results
