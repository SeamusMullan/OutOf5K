"""Tests for the main service."""

import json
from io import StringIO

from outof5k_parser.main import ServiceMain


def test_handle_ping():
    """Test the ping command handler."""
    service = ServiceMain()
    result = service._handle_ping({}, lambda p, m: None)

    assert "version" in result
    assert "demoparser_version" in result


def test_handle_shutdown():
    """Test the shutdown command handler."""
    service = ServiceMain()
    assert service._running is True

    result = service._handle_shutdown({}, lambda p, m: None)

    assert service._running is False
    assert "message" in result


def test_dispatch_unknown_command(capsys):
    """Test dispatching an unknown command."""
    service = ServiceMain()

    request = {"id": "test-123", "cmd": "unknown_command", "params": {}}
    service._dispatch(request)

    captured = capsys.readouterr()
    response = json.loads(captured.out.strip())

    assert response["status"] == "error"
    assert response["error"]["code"] == "UNKNOWN_COMMAND"
