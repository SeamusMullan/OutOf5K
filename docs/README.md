# OutOf5K Documentation

> Technical documentation for OutOf5K - A CS2 gameplay review and improvement application.

## Overview

This documentation provides comprehensive technical details for developing, understanding, and contributing to OutOf5K. It is designed to be useful both as in-repo reference and as a compiled PDF document.

## Documentation Structure

```
docs/
├── architecture/          # System architecture and design
│   ├── overview.md        # High-level architecture narrative
│   ├── c4/                # C4 model diagrams (PlantUML)
│   └── uml/               # UML diagrams (Sequence, Class, State)
├── data-flow/             # Data flow and protocols
│   ├── ipc-protocol.md    # Tauri ↔ Python communication
│   ├── demo-parsing-pipeline.md
│   ├── event-system.md
│   └── query-patterns.md
├── patterns/              # Design patterns glossary
├── adr/                   # Architecture Decision Records
├── guides/                # Developer guides
└── latex/                 # LaTeX source for PDF generation
```

## Quick Links

### Architecture
- [Architecture Overview](./architecture/overview.md) - Start here for system understanding
- [C4 Diagrams](./architecture/c4/) - Visual system architecture
- [UML Diagrams](./architecture/uml/) - Detailed design diagrams

### Technical Reference
- [IPC Protocol](./data-flow/ipc-protocol.md) - Communication between components
- [Demo Parsing Pipeline](./data-flow/demo-parsing-pipeline.md) - How CS2 demos are processed
- [Design Patterns](./patterns/README.md) - Patterns used in the codebase

### Development
- [Developer Setup](./guides/developer-setup.md) - Getting started
- [Project Structure](./guides/project-structure.md) - Codebase organization
- [Contributing](./guides/contributing.md) - How to contribute

### Decisions
- [Architecture Decision Records](./adr/) - Why we made certain choices

## Building Documentation

### Quick Start

```bash
cd docs

# Generate diagrams and build PDF (downloads PlantUML automatically)
./build.sh all

# Or just generate diagrams
./build.sh diagrams

# See all options
./build.sh help
```

### Requirements

- **Java** (for PlantUML diagram generation)
- **LaTeX** (for PDF generation) - optional, only needed for PDF

### Viewing Diagrams

All architecture and UML diagrams are written in PlantUML format (`.puml` files).

**Option 1: VS Code Extension** (Recommended)
```bash
code --install-extension jebbs.plantuml
```

**Option 2: Generate Images**
```bash
./build.sh diagrams   # Generates PNGs in latex/figures/
./build.sh svg        # Generates SVGs for web use
```

**Option 3: Online Viewer**
- Copy diagram content to [PlantUML Web Server](http://www.plantuml.com/plantuml/uml/)

### Building the PDF

The complete documentation can be compiled into a Tufte-style PDF:

```bash
./build.sh all    # Diagrams + PDF
./build.sh pdf    # PDF only (if diagrams already generated)
```

Output: `docs/latex/main.pdf`

See [latex/README.md](./latex/README.md) for detailed build instructions and troubleshooting.

## Tech Stack Summary

| Component | Technology | Purpose |
|-----------|------------|---------|
| Desktop Framework | Tauri v2 | Native desktop app shell |
| Frontend | Svelte 5 + TypeScript | User interface |
| Styling | TailwindCSS + shadcn-svelte | UI components |
| 2D Visualization | Pixi.js | Map rendering, heatmaps |
| Backend | Rust (Tauri) | System integration, DB access |
| Demo Parser | Python + demoparser2 | CS2 demo file parsing |
| Database | SQLite | Local data storage |
| Diagrams | PlantUML | Technical diagrams |
| Documentation | LaTeX (Tufte) | PDF generation |

## Contributing to Documentation

When updating documentation:

1. **Diagrams**: Edit `.puml` files, regenerate images before committing
2. **Markdown**: Keep in sync with LaTeX chapters
3. **ADRs**: Follow the [ADR template](./adr/template.md) for new decisions
4. **Build PDF**: Ensure `make pdf` succeeds before submitting

## License

This documentation is part of the OutOf5K project and is released under the same license.
