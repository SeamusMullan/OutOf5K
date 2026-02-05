# Developer Setup Guide

This guide will help you set up your development environment for OutOf5K.

## Prerequisites

### Required Software

| Software | Version | Purpose |
|----------|---------|---------|
| **Node.js** | 20.x LTS | Frontend development |
| **Rust** | 1.75+ | Tauri backend |
| **Python** | 3.11+ | Demo parsing service |
| **pnpm** | 8.x | Package management |

### Optional (Recommended)

| Software | Purpose |
|----------|---------|
| **VS Code** | Recommended IDE |
| **Java 11+** | PlantUML diagram generation |
| **PlantUML** | Viewing/generating diagrams |

## Installation

### 1. Install System Dependencies

#### macOS

```bash
# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install node@20 rustup python@3.11 pnpm

# Set up Rust
rustup-init
source $HOME/.cargo/env

# Tauri dependencies
xcode-select --install
```

#### Ubuntu/Debian

```bash
# Update package list
sudo apt update

# Install Node.js 20
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Python 3.11
sudo apt install -y python3.11 python3.11-venv python3-pip

# Install pnpm
npm install -g pnpm

# Tauri dependencies
sudo apt install -y libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Windows

```powershell
# Install Chocolatey if not present
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install dependencies
choco install nodejs-lts rustup python311 pnpm

# Initialize Rust
rustup-init

# Tauri dependencies (via Visual Studio Build Tools)
# Download and install from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
# Select "Desktop development with C++" workload
```

### 2. Clone the Repository

```bash
git clone https://github.com/your-org/outof5k.git
cd outof5k
```

### 3. Install Dependencies

```bash
# Install frontend dependencies
pnpm install

# Set up Python virtual environment
cd python
python3.11 -m venv venv
source venv/bin/activate  # On Windows: .\venv\Scripts\activate
pip install -r requirements.txt
pip install -r requirements-dev.txt
cd ..
```

### 4. Verify Installation

```bash
# Check versions
node --version    # Should be 20.x
rustc --version   # Should be 1.75+
python --version  # Should be 3.11+
pnpm --version    # Should be 8.x

# Run the app in development mode
pnpm tauri dev
```

## IDE Setup

### VS Code (Recommended)

Install the following extensions:

```bash
# Essential
code --install-extension svelte.svelte-vscode
code --install-extension rust-lang.rust-analyzer
code --install-extension ms-python.python
code --install-extension bradlc.vscode-tailwindcss

# Recommended
code --install-extension esbenp.prettier-vscode
code --install-extension dbaeumer.vscode-eslint
code --install-extension tauri-apps.tauri-vscode
code --install-extension jebbs.plantuml
```

Recommended VS Code settings (`.vscode/settings.json`):

```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[python]": {
    "editor.defaultFormatter": "ms-python.python"
  },
  "svelte.enable-ts-plugin": true,
  "typescript.tsdk": "node_modules/typescript/lib",
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

### JetBrains IDEs

For IntelliJ IDEA, WebStorm, or PyCharm:

1. Install the **Rust** plugin
2. Install the **Svelte** plugin
3. Enable TypeScript service for `.svelte` files
4. Configure Python interpreter to use the virtual environment

## Development Workflow

### Running the Application

```bash
# Development mode (hot reload)
pnpm tauri dev

# Build for production
pnpm tauri build
```

### Running Tests

```bash
# Frontend tests
pnpm test

# Rust tests
cd src-tauri && cargo test

# Python tests
cd python && pytest
```

### Linting and Formatting

```bash
# Frontend
pnpm lint
pnpm format

# Rust
cd src-tauri && cargo fmt && cargo clippy

# Python
cd python && black . && ruff check .
```

### Generating Diagrams

```bash
# Requires Java and PlantUML
cd docs/latex
make diagrams

# Or manually:
java -jar plantuml.jar ../architecture/**/*.puml -tpng
```

## Database

### Location

The SQLite database is stored at:
- **macOS**: `~/Library/Application Support/com.outof5k.app/outof5k.db`
- **Linux**: `~/.local/share/com.outof5k.app/outof5k.db`
- **Windows**: `%APPDATA%\com.outof5k.app\outof5k.db`

### Migrations

Migrations run automatically on app startup. To run manually:

```bash
cd src-tauri && cargo run --bin migrate
```

### Reset Database

Delete the database file to reset:

```bash
# macOS
rm ~/Library/Application\ Support/com.outof5k.app/outof5k.db

# Linux
rm ~/.local/share/com.outof5k.app/outof5k.db
```

## Python Service

### Development Mode

The Python service runs automatically when you start the Tauri app. For standalone development:

```bash
cd python
source venv/bin/activate
python -m outof5k_parser
```

### Adding Dependencies

```bash
cd python
pip install new-package
pip freeze > requirements.txt
```

## Common Issues

### Issue: Tauri build fails on Windows

**Solution**: Ensure Visual Studio Build Tools are installed with the "Desktop development with C++" workload.

### Issue: Python service not found

**Solution**: Ensure the virtual environment is activated and the Python path is correct in the Tauri configuration.

### Issue: WebView not rendering

**Solution**: 
- macOS: Ensure Xcode command line tools are installed
- Linux: Install webkit2gtk development packages
- Windows: Ensure WebView2 runtime is installed

### Issue: Hot reload not working

**Solution**: Check that the development server is running on the expected port (usually 1420).

## Next Steps

- Read the [Project Structure Guide](./project-structure.md) to understand the codebase
- Review the [Architecture Overview](../architecture/overview.md)
- Check the [Contributing Guide](./contributing.md) before making changes
