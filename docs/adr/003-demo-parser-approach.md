# ADR-003: Python Subprocess for Demo Parsing

## Status

Accepted

## Date

2024-01-15

## Context

Parsing CS2 demo files is a core function of OutOf5K. Demo files are binary, proprietary formats that require specialized libraries to decode. Options for demo parsing:

1. **demoparser2** (Python): The most mature and feature-complete CS2 demo parser
2. **Source 2 Demo Parser (Rust)**: Native Rust parser, less mature
3. **Node.js parsers**: Various npm packages, varying quality

The key considerations:
- Parser maturity and CS2 support
- Performance for large demo files (100MB+)
- Integration complexity with Tauri
- Future extensibility (ML/AI features)

## Decision

We will use **Python as a subprocess** running demoparser2, communicating with the Tauri backend via stdin/stdout JSON protocol.

## Consequences

### Positive

1. **Best Parser Available**: demoparser2 is the most mature, actively maintained CS2 demo parser. It handles edge cases and updates quickly when Valve changes the demo format.

2. **Python Ecosystem**: Access to NumPy, pandas, and ML libraries (PyTorch, scikit-learn) for future AI coaching features.

3. **Process Isolation**: If the parser crashes on a corrupted demo, it doesn't take down the entire application.

4. **Independent Development**: Parser can be developed and tested independently of the main application.

5. **Parallelization**: Can spawn multiple Python processes for concurrent demo parsing.

6. **Simple Protocol**: JSON over stdin/stdout is debuggable, language-agnostic, and well-understood.

### Negative

1. **Process Overhead**: Spawning a subprocess has overhead compared to in-process execution. Mitigated by keeping the process running and reusing it.

2. **Distribution Complexity**: Need to either:
   - Bundle Python with the application
   - Require users to have Python installed
   - Use PyInstaller to create a standalone executable

3. **Memory Usage**: Two separate processes (Tauri + Python) means higher total memory footprint.

4. **IPC Latency**: Serializing/deserializing JSON adds latency. Acceptable for batch operations, may need optimization for real-time features.

5. **Error Handling Complexity**: Need to handle subprocess crashes, timeouts, and communication errors.

### Neutral

1. **Two Languages**: The codebase uses both Rust and Python. Developers need familiarity with both, but the concerns are well-separated.

## Alternatives Considered

### Alternative 1: Native Rust Parser

Building or using a Rust-native demo parser.

**Why Rejected:**
- No mature Rust parser for CS2 exists
- Building one would be a massive undertaking
- Would miss updates when Valve changes the format
- demoparser2 has years of edge case handling we'd need to replicate

### Alternative 2: WebAssembly Compilation

Compiling Python/demoparser2 to WASM.

**Why Rejected:**
- demoparser2 uses native dependencies that don't compile to WASM
- Performance would likely be worse than native Python
- Increased complexity without clear benefit

### Alternative 3: Embedded Python (PyO3)

Running Python directly in the Rust process using PyO3.

**Why Rejected:**
- GIL limitations would prevent true parallelism
- More complex error handling (Python exceptions in Rust)
- Crashes in Python code could crash the entire application
- Distribution complexity is similar (still need Python runtime)

### Alternative 4: Node.js Parser

Using JavaScript-based parsers via Tauri's Node.js support.

**Why Rejected:**
- Available Node.js parsers are less mature than demoparser2
- Would lose access to Python's scientific computing ecosystem
- No clear advantage over the Python subprocess approach

## Implementation Details

### Process Lifecycle

```
App Start → Spawn Python → Ready Signal → Command Loop → App Exit → Shutdown
```

### Distribution Strategy

We will use **PyInstaller** to create a standalone Python executable:

```bash
pyinstaller --onefile --name outof5k-parser src/__main__.py
```

This produces a single binary that includes Python and all dependencies, avoiding the need for users to install Python.

### Protocol Specification

See [IPC Protocol Documentation](../data-flow/ipc-protocol.md) for the complete specification.

## References

- [demoparser2 GitHub](https://github.com/LaihoE/demoparser)
- [PyO3 User Guide](https://pyo3.rs/)
- [PyInstaller Documentation](https://pyinstaller.org/)
- [Tauri Process Management](https://tauri.app/v1/api/js/shell/)
