"""Main service loop for the demo parser.

This module implements the JSON-over-stdin/stdout protocol for communication
with the Tauri backend. See docs/data-flow/ipc-protocol.md for the full spec.
"""

from __future__ import annotations

import json
import sys
import traceback
from typing import TYPE_CHECKING, Any, Callable

from .handlers import (
    handle_analyze,
    handle_get_metadata,
    handle_get_positions,
    handle_parse,
)
from .models.responses import ErrorInfo, ErrorResponse, SuccessResponse

if TYPE_CHECKING:
    from collections.abc import Iterator


class ServiceMain:
    """Main service class that handles the IPC protocol."""

    def __init__(self) -> None:
        """Initialize the service with command handlers."""
        self.handlers: dict[str, Callable[[dict[str, Any], Callable[..., None]], Any]] = {
            "ping": self._handle_ping,
            "parse": handle_parse,
            "get_metadata": handle_get_metadata,
            "get_positions": handle_get_positions,
            "analyze": handle_analyze,
            "shutdown": self._handle_shutdown,
        }
        self._running = True

    def run(self) -> None:
        """Run the main service loop.

        Sends a ready signal, then processes commands from stdin until shutdown.
        """
        # Signal ready to the Tauri backend
        self._send_response({"status": "ready"})

        for line in self._read_lines():
            if not self._running:
                break

            try:
                request = json.loads(line)
                self._dispatch(request)
            except json.JSONDecodeError as e:
                self._send_error_response(
                    request_id=None,
                    code="INVALID_JSON",
                    message=f"Failed to parse JSON: {e}",
                )
            except Exception as e:
                self._send_error_response(
                    request_id=None,
                    code="INTERNAL_ERROR",
                    message=str(e),
                    details={"traceback": traceback.format_exc()},
                )

    def _read_lines(self) -> Iterator[str]:
        """Read lines from stdin."""
        for line in sys.stdin:
            yield line.strip()

    def _dispatch(self, request: dict[str, Any]) -> None:
        """Dispatch a request to the appropriate handler."""
        request_id = request.get("id")
        cmd = request.get("cmd")
        params = request.get("params", {})

        if not cmd:
            self._send_error_response(
                request_id=request_id,
                code="INVALID_PARAMS",
                message="Missing 'cmd' field in request",
            )
            return

        handler = self.handlers.get(cmd)
        if not handler:
            self._send_error_response(
                request_id=request_id,
                code="UNKNOWN_COMMAND",
                message=f"Unknown command: {cmd}",
            )
            return

        try:
            # Create a progress callback for this request
            def progress_callback(progress: int, message: str) -> None:
                self._send_progress(request_id, progress, message)

            result = handler(params, progress_callback)
            self._send_success_response(request_id, result)
        except FileNotFoundError as e:
            self._send_error_response(
                request_id=request_id,
                code="FILE_NOT_FOUND",
                message=str(e),
            )
        except PermissionError as e:
            self._send_error_response(
                request_id=request_id,
                code="FILE_READ_ERROR",
                message=str(e),
            )
        except Exception as e:
            self._send_error_response(
                request_id=request_id,
                code="INTERNAL_ERROR",
                message=str(e),
                details={"traceback": traceback.format_exc()},
            )

    def _handle_ping(
        self, _params: dict[str, Any], _progress: Callable[..., None]
    ) -> dict[str, str]:
        """Handle the ping command."""
        try:
            import demoparser2

            demoparser_version = demoparser2.__version__
        except (ImportError, AttributeError):
            demoparser_version = "unknown"

        from . import __version__

        return {
            "version": __version__,
            "demoparser_version": demoparser_version,
        }

    def _handle_shutdown(
        self, _params: dict[str, Any], _progress: Callable[..., None]
    ) -> dict[str, str]:
        """Handle the shutdown command."""
        self._running = False
        return {"message": "Shutting down"}

    def _send_response(self, response: dict[str, Any]) -> None:
        """Send a response to stdout."""
        print(json.dumps(response), flush=True)

    def _send_success_response(self, request_id: str | None, data: Any) -> None:
        """Send a success response."""
        response = SuccessResponse(id=request_id, data=data)
        self._send_response(response.model_dump(exclude_none=True))

    def _send_error_response(
        self,
        request_id: str | None,
        code: str,
        message: str,
        details: dict[str, Any] | None = None,
    ) -> None:
        """Send an error response."""
        error = ErrorInfo(code=code, message=message, details=details)
        response = ErrorResponse(id=request_id, error=error)
        self._send_response(response.model_dump(exclude_none=True))

    def _send_progress(self, request_id: str | None, progress: int, message: str) -> None:
        """Send a progress update."""
        response = {
            "id": request_id,
            "status": "progress",
            "progress": progress,
            "message": message,
        }
        self._send_response(response)
