"""Response models for IPC communication."""

from __future__ import annotations

from typing import Any, Literal

from pydantic import BaseModel


class ErrorInfo(BaseModel):
    """Error information for error responses."""

    code: str
    message: str
    details: dict[str, Any] | None = None


class SuccessResponse(BaseModel):
    """A successful response."""

    id: str | None = None
    status: Literal["success"] = "success"
    data: Any = None


class ErrorResponse(BaseModel):
    """An error response."""

    id: str | None = None
    status: Literal["error"] = "error"
    error: ErrorInfo


class ProgressResponse(BaseModel):
    """A progress update response."""

    id: str | None = None
    status: Literal["progress"] = "progress"
    progress: int
    message: str
