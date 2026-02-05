# Architecture Overview

## Introduction

OutOf5K is a desktop application designed to help Counter-Strike 2 players analyze their gameplay, track improvement over time, and learn from professional players. This document provides a high-level overview of the system architecture.

## Design Philosophy

### Core Principles

1. **Local-First**: All demo processing happens locally on the user's machine, eliminating server costs and preserving user privacy.

2. **Modular Architecture**: The system is composed of loosely coupled components that communicate through well-defined interfaces.

3. **Progressive Enhancement**: Start with essential features (demo parsing, basic stats) and progressively add advanced capabilities (3D replay, AI coaching).

4. **Performance-Conscious**: CS2 demo files can be large (100MB+). The architecture is designed to handle this efficiently through background processing and incremental parsing.

## System Context

OutOf5K operates as a standalone desktop application that interacts with several external systems:

- **Steam Platform**: User authentication and match history
- **HLTV**: Professional player statistics and match data
- **Faceit/ESEA**: Competitive match data and rankings
- **CS2 Game**: Demo file source, potential in-game integration

```
┌─────────────────────────────────────────────────────────────────┐
│                         OutOf5K User                            │
│                    (CS2 Player seeking improvement)             │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      OutOf5K Desktop App                        │
│         Gameplay analysis, stats tracking, pro comparison       │
└─────────────────────────────────────────────────────────────────┘
           │              │              │              │
           ▼              ▼              ▼              ▼
      ┌────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
      │ Steam  │    │  HLTV   │    │ Faceit/ │    │  CS2    │
      │  API   │    │  Data   │    │  ESEA   │    │ Demos   │
      └────────┘    └─────────┘    └─────────┘    └─────────┘
```

*See [C4 Context Diagram](./c4/1-context.puml) for the formal PlantUML version.*

## Container Architecture

The application is composed of four main containers:

### 1. Frontend (Svelte + TypeScript)

The user interface layer built with Svelte 5 and TypeScript. Responsibilities:
- Rendering UI components
- User interaction handling
- 2D map visualization (Pixi.js)
- Statistics charts and graphs
- State management via Svelte stores

### 2. Tauri Backend (Rust)

The desktop application shell and system integration layer. Responsibilities:
- Native window management
- File system access (demo files)
- SQLite database operations
- Python subprocess management
- System tray and notifications
- IPC command handlers

### 3. Python Service

A subprocess that handles computationally intensive demo parsing. Responsibilities:
- CS2 demo file parsing (demoparser2)
- Data extraction and transformation
- Heavy computational analysis
- Future: ML model inference

### 4. SQLite Database

Local persistent storage for all application data:
- Parsed demo metadata and events
- Player statistics and profiles
- User preferences and settings
- Cached external API data

```
┌─────────────────────────────────────────────────────────────────┐
│                      OutOf5K Desktop App                        │
├─────────────────────────────────────────────────────────────────┤
│  ┌────────────────────────────────────────────────────────┐    │
│  │              Frontend (Svelte + TypeScript)             │    │
│  │  Components │ Stores │ 2D Canvas │ Charts │ Router     │    │
│  └────────────────────────────────────────────────────────┘    │
│                              │                                  │
│                         Tauri IPC                               │
│                              │                                  │
│  ┌────────────────────────────────────────────────────────┐    │
│  │              Tauri Backend (Rust)                       │    │
│  │  Commands │ DB Access │ File System │ Python Bridge    │    │
│  └────────────────────────────────────────────────────────┘    │
│          │                                    │                 │
│          ▼                                    ▼                 │
│  ┌───────────────┐              ┌────────────────────────┐     │
│  │    SQLite     │              │    Python Service      │     │
│  │   Database    │              │  (Demo Parser)         │     │
│  └───────────────┘              └────────────────────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

*See [C4 Container Diagram](./c4/2-container.puml) for the formal PlantUML version.*

## Key Data Flows

### Demo Import Flow

1. User selects/drops demo file(s)
2. Frontend sends import command via Tauri IPC
3. Rust backend validates file and spawns Python process
4. Python parses demo, extracts events, sends JSON back
5. Rust backend stores parsed data in SQLite
6. Frontend receives completion notification and refreshes UI

### Statistics Calculation

1. Frontend requests stats for a player/time period
2. Rust backend queries SQLite, aggregates data
3. Results returned to frontend for visualization

### External API Sync

1. User authenticates with Steam
2. Rust backend fetches match history, player data
3. Data cached in SQLite with TTL
4. Pro comparison data fetched from HLTV/Faceit
5. Frontend displays comparisons

## Technology Rationale

| Decision | Rationale |
|----------|-----------|
| **Tauri** | Small bundle size (~20MB vs Electron's 150MB+), better performance, native feel |
| **Svelte** | Minimal boilerplate, reactive by default, excellent TypeScript support |
| **Python subprocess** | demoparser2 is the best CS2 demo parser available; Python ecosystem for future ML |
| **SQLite** | Zero-config, file-based, perfect for desktop apps, fast for local queries |
| **Pixi.js** | High-performance 2D rendering for smooth map visualization and heatmaps |

## Security Considerations

1. **Local Processing**: Demo files never leave the user's machine
2. **OAuth Only**: Steam authentication uses OAuth, no password handling
3. **API Key Protection**: External API keys stored securely via Tauri's secure storage
4. **Input Validation**: All file inputs validated before processing

## Scalability & Performance

### Demo Parsing
- Large demos parsed in background thread
- Progress reported incrementally to UI
- Parsed data indexed for fast queries

### Data Storage
- SQLite with proper indexes
- Pagination for large result sets
- Lazy loading for detailed event data

### Visualization
- Canvas-based rendering for smooth performance
- Level-of-detail for heatmaps
- Virtualized lists for large demo libraries

## Future Architecture Considerations

### 3D Replay System (Phase 4)
- Evaluate Three.js vs embedded game engine
- Map asset extraction and licensing
- Performance optimization for real-time playback

### Cloud Sync (Phase 5)
- Optional backend service for team features
- End-to-end encryption for user data
- Conflict resolution for offline changes

### AI Coaching (Phase 4)
- Local LLM integration (Ollama) for privacy
- Pattern recognition models
- Training pipeline for CS2-specific insights

## Related Documents

- [C4 Diagrams](./c4/) - Detailed architecture diagrams
- [UML Diagrams](./uml/) - Sequence, class, and state diagrams
- [Data Flow Documentation](../data-flow/) - Detailed data flow specifications
- [Architecture Decision Records](../adr/) - Decision rationale
