# IPC Protocol - Tauri to Python Communication

## Overview

Communication between the Tauri backend (Rust) and the Python demo parsing service uses a simple JSON-based protocol over stdin/stdout. This design allows for process isolation, easy debugging, and language-agnostic communication.

## Transport Layer

```
┌─────────────────┐           ┌─────────────────┐
│  Tauri Backend  │           │  Python Service │
│     (Rust)      │           │                 │
├─────────────────┤           ├─────────────────┤
│                 │  stdin    │                 │
│   write_line() ─┼──────────►│ readline()      │
│                 │           │                 │
│   read_line() ◄─┼───────────┼─ write_line()   │
│                 │  stdout   │                 │
└─────────────────┘           └─────────────────┘
```

### Connection Lifecycle

1. **Startup**: Tauri spawns Python process with specific working directory
2. **Ready Signal**: Python sends `{"status": "ready"}` on stdout
3. **Command Loop**: Tauri sends commands, Python responds
4. **Shutdown**: Tauri sends shutdown command, waits for process exit

### Message Framing

Each message is a single line of JSON, terminated by newline (`\n`). This allows for simple line-based parsing on both ends.

```
{"id": "123", "cmd": "parse", "params": {...}}\n
```

## Message Format

### Request (Tauri → Python)

```typescript
interface Request {
  id: string;         // Unique request identifier (UUID)
  cmd: string;        // Command name
  params: object;     // Command-specific parameters
}
```

### Response (Python → Tauri)

```typescript
interface Response {
  id: string;         // Matching request ID
  status: "success" | "error" | "progress";
  data?: object;      // Response payload (for success)
  error?: ErrorInfo;  // Error details (for error)
  progress?: number;  // Progress percentage (0-100, for progress)
  message?: string;   // Human-readable message
}

interface ErrorInfo {
  code: string;       // Error code (e.g., "FILE_NOT_FOUND")
  message: string;    // Error message
  details?: object;   // Additional error context
}
```

## Commands

### `ping` - Health Check

Tests that the Python service is responsive.

**Request:**
```json
{
  "id": "abc-123",
  "cmd": "ping",
  "params": {}
}
```

**Response:**
```json
{
  "id": "abc-123",
  "status": "success",
  "data": {
    "version": "1.0.0",
    "demoparser_version": "0.7.0"
  }
}
```

---

### `parse` - Parse Demo File

Parses a CS2 demo file and extracts all data.

**Request:**
```json
{
  "id": "abc-123",
  "cmd": "parse",
  "params": {
    "file_path": "/path/to/demo.dem",
    "options": {
      "extract_positions": true,
      "position_interval": 16,
      "extract_voice": false
    }
  }
}
```

**Progress Responses:**
```json
{
  "id": "abc-123",
  "status": "progress",
  "progress": 10,
  "message": "Extracting header..."
}
```

```json
{
  "id": "abc-123",
  "status": "progress",
  "progress": 50,
  "message": "Processing events..."
}
```

**Success Response:**
```json
{
  "id": "abc-123",
  "status": "success",
  "data": {
    "metadata": {
      "map_name": "de_dust2",
      "played_at": "2024-01-15T14:30:00Z",
      "duration_ticks": 256000,
      "tickrate": 64,
      "server_name": "Valve CS2 Server"
    },
    "players": [
      {
        "steam_id": "76561198012345678",
        "name": "PlayerOne",
        "team": "CT"
      }
    ],
    "rounds": [
      {
        "number": 1,
        "winner": "CT",
        "win_reason": "BOMB_DEFUSED",
        "start_tick": 1000,
        "end_tick": 5000
      }
    ],
    "events": [
      {
        "tick": 1500,
        "round": 1,
        "type": "KILL",
        "data": {
          "attacker_id": "76561198012345678",
          "victim_id": "76561198087654321",
          "weapon": "ak47",
          "headshot": true
        }
      }
    ],
    "stats": {
      "players": {
        "76561198012345678": {
          "kills": 25,
          "deaths": 15,
          "assists": 5,
          "adr": 85.5
        }
      }
    }
  }
}
```

**Error Response:**
```json
{
  "id": "abc-123",
  "status": "error",
  "error": {
    "code": "PARSE_ERROR",
    "message": "Failed to parse demo file",
    "details": {
      "reason": "Unsupported demo version",
      "version_found": "4"
    }
  }
}
```

---

### `get_metadata` - Get Demo Metadata Only

Quickly extracts just the header/metadata without full parsing.

**Request:**
```json
{
  "id": "abc-123",
  "cmd": "get_metadata",
  "params": {
    "file_path": "/path/to/demo.dem"
  }
}
```

**Response:**
```json
{
  "id": "abc-123",
  "status": "success",
  "data": {
    "map_name": "de_mirage",
    "played_at": "2024-01-15T14:30:00Z",
    "duration_ticks": 256000,
    "tickrate": 64
  }
}
```

---

### `get_positions` - Get Player Positions for Tick Range

Retrieves position data for visualization.

**Request:**
```json
{
  "id": "abc-123",
  "cmd": "get_positions",
  "params": {
    "file_path": "/path/to/demo.dem",
    "start_tick": 1000,
    "end_tick": 2000,
    "interval": 8
  }
}
```

**Response:**
```json
{
  "id": "abc-123",
  "status": "success",
  "data": {
    "positions": [
      {
        "tick": 1000,
        "players": {
          "76561198012345678": {
            "x": 1024.5,
            "y": -512.3,
            "z": 64.0,
            "view_x": 45.0,
            "view_y": 0.0,
            "health": 100,
            "armor": 100
          }
        }
      }
    ]
  }
}
```

---

### `analyze` - Run Analysis on Parsed Data

Runs specific analyzers on demo data.

**Request:**
```json
{
  "id": "abc-123",
  "cmd": "analyze",
  "params": {
    "file_path": "/path/to/demo.dem",
    "analyzers": ["highlights", "utility", "positions"]
  }
}
```

**Response:**
```json
{
  "id": "abc-123",
  "status": "success",
  "data": {
    "highlights": [
      {
        "type": "ACE",
        "round": 5,
        "player_id": "76561198012345678",
        "start_tick": 12000,
        "end_tick": 14000
      }
    ],
    "utility": {
      "grenades_thrown": 45,
      "damage_dealt": 234,
      "enemies_flashed": 12
    }
  }
}
```

---

### `shutdown` - Graceful Shutdown

Requests the Python service to shut down gracefully.

**Request:**
```json
{
  "id": "abc-123",
  "cmd": "shutdown",
  "params": {}
}
```

**Response:**
```json
{
  "id": "abc-123",
  "status": "success",
  "message": "Shutting down"
}
```

## Error Codes

| Code | Description |
|------|-------------|
| `FILE_NOT_FOUND` | Demo file does not exist |
| `FILE_READ_ERROR` | Cannot read demo file |
| `PARSE_ERROR` | Demo parsing failed |
| `UNSUPPORTED_VERSION` | Demo version not supported |
| `INVALID_PARAMS` | Invalid request parameters |
| `INTERNAL_ERROR` | Unexpected internal error |
| `TIMEOUT` | Operation timed out |

## Implementation Notes

### Rust Side (Tauri)

```rust
use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader, Write};

pub struct PythonBridge {
    process: Child,
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
}

impl PythonBridge {
    pub async fn send_command(&mut self, cmd: &str, params: Value) -> Result<Response> {
        let request = Request {
            id: Uuid::new_v4().to_string(),
            cmd: cmd.to_string(),
            params,
        };
        
        // Send request
        writeln!(self.stdin, "{}", serde_json::to_string(&request)?)?;
        self.stdin.flush()?;
        
        // Read responses until we get success/error
        loop {
            let mut line = String::new();
            self.stdout.read_line(&mut line)?;
            let response: Response = serde_json::from_str(&line)?;
            
            match response.status.as_str() {
                "progress" => {
                    // Emit progress event to frontend
                    self.emit_progress(&response);
                }
                "success" | "error" => {
                    return Ok(response);
                }
            }
        }
    }
}
```

### Python Side

```python
import sys
import json
from typing import Callable

class ServiceMain:
    def __init__(self):
        self.handlers = {
            "ping": self.handle_ping,
            "parse": self.handle_parse,
            "shutdown": self.handle_shutdown,
        }
    
    def run(self):
        # Signal ready
        self.send_response({"status": "ready"})
        
        for line in sys.stdin:
            try:
                request = json.loads(line)
                response = self.dispatch(request)
                self.send_response(response)
            except Exception as e:
                self.send_response({
                    "id": request.get("id"),
                    "status": "error",
                    "error": {"code": "INTERNAL_ERROR", "message": str(e)}
                })
    
    def send_response(self, response: dict):
        print(json.dumps(response), flush=True)
    
    def send_progress(self, request_id: str, progress: int, message: str):
        self.send_response({
            "id": request_id,
            "status": "progress",
            "progress": progress,
            "message": message
        })
```

## Concurrency

The protocol is designed for sequential request-response:
- Tauri sends one request, waits for final response
- Python can send multiple progress updates before final response
- No concurrent requests to simplify implementation

For parallel parsing, Tauri spawns multiple Python processes.
