"""Command handlers for the demo parser service."""

from .analyze import handle_analyze
from .metadata import handle_get_metadata
from .parse import handle_parse
from .positions import handle_get_positions

__all__ = [
    "handle_analyze",
    "handle_get_metadata",
    "handle_parse",
    "handle_get_positions",
]
