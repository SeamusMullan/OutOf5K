# Development Workflow

This guide covers day-to-day development practices for OutOf5K.

## Branch Strategy

We use a simplified Git Flow:

```
main                    # Production-ready code
├── develop             # Integration branch
│   ├── feature/*       # New features
│   ├── fix/*           # Bug fixes
│   └── docs/*          # Documentation updates
└── release/*           # Release preparation
```

### Branch Naming

| Type | Pattern | Example |
|------|---------|---------|
| Feature | `feature/short-description` | `feature/demo-import` |
| Bug Fix | `fix/issue-description` | `fix/parse-crash-on-empty` |
| Documentation | `docs/what-changed` | `docs/add-setup-guide` |
| Release | `release/version` | `release/1.0.0` |

## Development Cycle

### Starting New Work

```bash
# Ensure you're on develop and up to date
git checkout develop
git pull origin develop

# Create feature branch
git checkout -b feature/my-new-feature

# Start development server
pnpm tauri dev
```

### Making Changes

1. **Write code** following our style guides
2. **Test locally** - both manual and automated
3. **Commit frequently** with clear messages

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, no code change
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(parser): add support for CS2 demo format v4

fix(ui): prevent crash when demo list is empty

docs(readme): update installation instructions

refactor(db): extract repository pattern from service layer
```

### Submitting Changes

```bash
# Ensure tests pass
pnpm test
cd src-tauri && cargo test
cd python && pytest

# Push and create PR
git push origin feature/my-new-feature
```

## Testing

### Frontend Tests

```bash
# Run all tests
pnpm test

# Run with coverage
pnpm test:coverage

# Run specific test file
pnpm test src/lib/stores/demos.test.ts

# Watch mode
pnpm test:watch
```

### Rust Tests

```bash
cd src-tauri

# Run all tests
cargo test

# Run specific test
cargo test test_demo_parsing

# With output
cargo test -- --nocapture
```

### Python Tests

```bash
cd python
source venv/bin/activate

# Run all tests
pytest

# With coverage
pytest --cov=outof5k_parser

# Specific test file
pytest tests/test_parser.py

# Verbose output
pytest -v
```

### End-to-End Tests

```bash
# Run E2E tests (requires built app)
pnpm tauri build
pnpm test:e2e
```

## Code Quality

### Linting

```bash
# Frontend (ESLint + Svelte)
pnpm lint
pnpm lint:fix  # Auto-fix issues

# Rust (Clippy)
cd src-tauri
cargo clippy
cargo clippy --fix  # Auto-fix

# Python (Ruff)
cd python
ruff check .
ruff check --fix .  # Auto-fix
```

### Formatting

```bash
# Frontend (Prettier)
pnpm format
pnpm format:check  # Check only

# Rust
cd src-tauri
cargo fmt
cargo fmt --check  # Check only

# Python (Black)
cd python
black .
black --check .  # Check only
```

### Pre-commit Hooks

We use Husky for pre-commit hooks:

```bash
# Hooks run automatically on commit
# To skip (not recommended):
git commit --no-verify
```

Hooks include:
- Lint staged files
- Run relevant tests
- Check formatting

## Debugging

### Frontend Debugging

1. Open DevTools in the app: `Cmd+Option+I` (Mac) or `Ctrl+Shift+I` (Windows/Linux)
2. Use `console.log()` or Svelte DevTools extension
3. React-style debugging with breakpoints in Sources tab

### Rust Debugging

**VS Code:**
1. Install CodeLLDB extension
2. Add launch configuration:

```json
{
  "type": "lldb",
  "request": "launch",
  "name": "Debug Tauri",
  "cargo": {
    "args": ["build", "--manifest-path=src-tauri/Cargo.toml"]
  },
  "args": [],
  "cwd": "${workspaceFolder}"
}
```

**Command line:**
```bash
cd src-tauri
RUST_BACKTRACE=1 cargo run
```

### Python Debugging

**VS Code:**
1. Set Python interpreter to venv
2. Add launch configuration:

```json
{
  "name": "Python: Parser",
  "type": "python",
  "request": "launch",
  "module": "outof5k_parser",
  "cwd": "${workspaceFolder}/python"
}
```

**Command line:**
```bash
cd python
python -m pdb -m outof5k_parser
```

### Database Debugging

```bash
# Open SQLite CLI
sqlite3 ~/Library/Application\ Support/com.outof5k.app/outof5k.db

# Common queries
.tables                          # List tables
.schema demos                    # Show table schema
SELECT COUNT(*) FROM demos;      # Count records
```

## Building

### Development Build

```bash
pnpm tauri dev
```

### Production Build

```bash
# Build for current platform
pnpm tauri build

# Output locations:
# macOS: src-tauri/target/release/bundle/dmg/
# Linux: src-tauri/target/release/bundle/deb/ (or appimage/)
# Windows: src-tauri/target/release/bundle/msi/
```

### Building Python Executable

```bash
cd python
pip install pyinstaller
pyinstaller --onefile --name outof5k-parser src/outof5k_parser/__main__.py

# Output: dist/outof5k-parser
```

## Documentation

### Updating Docs

1. Edit Markdown files in `docs/`
2. For diagrams, edit `.puml` files and regenerate images
3. Preview locally before committing

### Generating Diagram Images

```bash
cd docs/latex
make diagrams
```

### Building PDF Documentation

```bash
cd docs/latex
make pdf

# Output: docs/latex/main.pdf
```

## Environment Variables

### Development

Create `.env.local` in the root (not committed):

```bash
# API keys for testing
STEAM_API_KEY=your_key_here
HLTV_API_KEY=your_key_here

# Development flags
DEBUG=true
LOG_LEVEL=debug
```

### Tauri Environment

In `src-tauri/.env` (not committed):

```bash
DATABASE_URL=sqlite:///path/to/dev.db
```

## Performance Profiling

### Frontend

```bash
# Bundle analyzer
pnpm analyze
```

### Rust

```bash
cd src-tauri

# CPU profiling
cargo build --release
# Use Instruments (macOS), perf (Linux), or cargo-flamegraph
cargo flamegraph

# Memory profiling with heaptrack (Linux)
heaptrack ./target/release/outof5k
```

### Python

```bash
cd python

# cProfile
python -m cProfile -o profile.stats -m outof5k_parser

# Visualize
pip install snakeviz
snakeviz profile.stats
```

## Useful Commands Reference

```bash
# Start dev server
pnpm tauri dev

# Run all tests
pnpm test && cd src-tauri && cargo test && cd ../python && pytest

# Lint everything
pnpm lint && cd src-tauri && cargo clippy && cd ../python && ruff check .

# Format everything
pnpm format && cd src-tauri && cargo fmt && cd ../python && black .

# Build production
pnpm tauri build

# Clean all build artifacts
rm -rf build node_modules src-tauri/target python/__pycache__ python/dist

# Regenerate dependencies
pnpm install && cd src-tauri && cargo build && cd ../python && pip install -r requirements.txt
```
