# Project Structure

This document explains the organization of the OutOf5K codebase.

## Overview

```
outof5k/
├── src/                    # Svelte frontend
├── src-tauri/              # Rust/Tauri backend
├── python/                 # Python demo parser service
├── docs/                   # Documentation
├── assets/                 # Static assets
└── [config files]          # Root configuration
```

## Frontend (`src/`)

The Svelte frontend application.

```
src/
├── app.html                # HTML template
├── app.css                 # Global styles (Tailwind imports)
├── lib/                    # Shared code
│   ├── components/         # Reusable UI components
│   │   ├── ui/             # Base UI components (shadcn-svelte)
│   │   │   ├── button/
│   │   │   ├── card/
│   │   │   └── ...
│   │   ├── demo/           # Demo-related components
│   │   │   ├── DemoCard.svelte
│   │   │   ├── DemoList.svelte
│   │   │   └── ...
│   │   ├── map/            # Map visualization components
│   │   │   ├── MapCanvas.svelte
│   │   │   ├── PlayerMarker.svelte
│   │   │   └── ...
│   │   ├── stats/          # Statistics components
│   │   └── layout/         # Layout components
│   │       ├── Sidebar.svelte
│   │       ├── Header.svelte
│   │       └── ...
│   ├── stores/             # Svelte stores (state management)
│   │   ├── demos.ts        # Demo list state
│   │   ├── playback.ts     # Playback/timeline state
│   │   ├── settings.ts     # User settings
│   │   └── auth.ts         # Authentication state
│   ├── api/                # Tauri IPC wrappers
│   │   ├── demos.ts        # Demo-related commands
│   │   ├── stats.ts        # Statistics commands
│   │   └── settings.ts     # Settings commands
│   ├── utils/              # Utility functions
│   │   ├── format.ts       # Formatting helpers
│   │   ├── map.ts          # Map coordinate helpers
│   │   └── ...
│   └── types/              # TypeScript type definitions
│       ├── demo.ts
│       ├── player.ts
│       └── events.ts
├── routes/                 # SvelteKit pages
│   ├── +layout.svelte      # Root layout
│   ├── +page.svelte        # Dashboard (home)
│   ├── demos/
│   │   ├── +page.svelte    # Demo library
│   │   └── [id]/
│   │       └── +page.svelte # Demo viewer
│   ├── stats/
│   │   └── +page.svelte    # Statistics view
│   ├── compare/
│   │   └── +page.svelte    # Pro comparison
│   └── settings/
│       └── +page.svelte    # Settings
└── static/                 # Static files (favicon, etc.)
```

### Key Frontend Patterns

**Components**: Use the `$lib` alias for imports:
```svelte
<script>
  import { Button } from '$lib/components/ui/button';
  import DemoCard from '$lib/components/demo/DemoCard.svelte';
</script>
```

**Stores**: Reactive state management:
```typescript
// stores/demos.ts
import { writable } from 'svelte/store';

export const demoStore = createDemoStore();
```

**API Layer**: Typed Tauri command wrappers:
```typescript
// api/demos.ts
import { invoke } from '@tauri-apps/api/tauri';

export async function importDemos(paths: string[]): Promise<ImportResult> {
  return invoke('import_demos', { paths });
}
```

## Tauri Backend (`src-tauri/`)

The Rust backend handling native functionality.

```
src-tauri/
├── Cargo.toml              # Rust dependencies
├── tauri.conf.json         # Tauri configuration
├── build.rs                # Build script
├── icons/                  # App icons
└── src/
    ├── main.rs             # Entry point
    ├── lib.rs              # Library root
    ├── commands/           # Tauri IPC command handlers
    │   ├── mod.rs
    │   ├── demo_commands.rs
    │   ├── stats_commands.rs
    │   └── settings_commands.rs
    ├── services/           # Business logic
    │   ├── mod.rs
    │   ├── demo_service.rs     # Demo import/management
    │   ├── stats_service.rs    # Statistics calculation
    │   ├── comparison_service.rs
    │   └── python_bridge.rs    # Python subprocess management
    ├── db/                 # Database layer
    │   ├── mod.rs
    │   ├── connection.rs   # Connection management
    │   ├── migrations/     # SQL migrations
    │   │   ├── mod.rs
    │   │   ├── 001_initial.sql
    │   │   └── ...
    │   └── repositories/   # Data access
    │       ├── mod.rs
    │       ├── demo_repository.rs
    │       ├── player_repository.rs
    │       └── stats_repository.rs
    ├── models/             # Domain models
    │   ├── mod.rs
    │   ├── demo.rs
    │   ├── player.rs
    │   ├── events.rs
    │   └── stats.rs
    ├── api/                # External API clients
    │   ├── mod.rs
    │   ├── steam.rs
    │   ├── hltv.rs
    │   └── faceit.rs
    ├── error.rs            # Error types
    └── state.rs            # Application state
```

### Key Backend Patterns

**Commands**: Tauri IPC entry points:
```rust
// commands/demo_commands.rs
#[tauri::command]
pub async fn import_demos(
    state: State<'_, AppState>,
    paths: Vec<String>,
) -> Result<ImportResult, CommandError> {
    state.demo_service.import_demos(&paths).await
}
```

**Services**: Business logic:
```rust
// services/demo_service.rs
pub struct DemoService {
    repository: DemoRepository,
    python_bridge: PythonBridge,
}

impl DemoService {
    pub async fn import_demos(&self, paths: &[String]) -> Result<Vec<ImportResult>> {
        // Validation, parsing, storage logic
    }
}
```

**Repositories**: Data access:
```rust
// db/repositories/demo_repository.rs
pub struct DemoRepository {
    pool: SqlitePool,
}

impl DemoRepository {
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Demo>> {
        sqlx::query_as!(Demo, "SELECT * FROM demos WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
    }
}
```

## Python Service (`python/`)

The demo parsing subprocess.

```
python/
├── pyproject.toml          # Project configuration
├── requirements.txt        # Production dependencies
├── requirements-dev.txt    # Development dependencies
├── src/
│   └── outof5k_parser/     # Package
│       ├── __init__.py
│       ├── __main__.py     # Entry point
│       ├── main.py         # Main service loop
│       ├── handlers/       # Command handlers
│       │   ├── __init__.py
│       │   ├── parse.py
│       │   ├── metadata.py
│       │   └── analyze.py
│       ├── parsing/        # Demo parsing logic
│       │   ├── __init__.py
│       │   ├── parser.py
│       │   ├── events.py
│       │   └── positions.py
│       ├── analysis/       # Analysis algorithms
│       │   ├── __init__.py
│       │   ├── stats.py
│       │   ├── highlights.py
│       │   └── positions.py
│       └── models/         # Data models (Pydantic)
│           ├── __init__.py
│           ├── demo.py
│           ├── events.py
│           └── responses.py
└── tests/                  # Test files
    ├── conftest.py
    ├── test_parser.py
    └── fixtures/           # Test demo files
```

### Key Python Patterns

**Entry Point**: Service initialization:
```python
# __main__.py
from .main import ServiceMain

if __name__ == "__main__":
    service = ServiceMain()
    service.run()
```

**Command Handlers**: Process incoming requests:
```python
# handlers/parse.py
from ..parsing import DemoParser
from ..models import ParsedDemo

def handle_parse(params: dict) -> ParsedDemo:
    parser = DemoParser(params["file_path"])
    return parser.parse(progress_callback=report_progress)
```

**Models**: Pydantic for validation:
```python
# models/demo.py
from pydantic import BaseModel

class DemoMetadata(BaseModel):
    map_name: str
    played_at: datetime
    duration_ticks: int
    tickrate: int
```

## Documentation (`docs/`)

```
docs/
├── README.md               # Documentation index
├── architecture/           # Architecture docs
│   ├── overview.md
│   ├── c4/                 # C4 PlantUML diagrams
│   └── uml/                # UML diagrams
├── data-flow/              # Data flow specs
├── patterns/               # Design patterns
├── adr/                    # Architecture decisions
├── guides/                 # Developer guides
└── latex/                  # PDF generation
```

## Configuration Files

| File | Purpose |
|------|---------|
| `package.json` | Frontend dependencies, scripts |
| `pnpm-lock.yaml` | Locked frontend dependencies |
| `svelte.config.js` | SvelteKit configuration |
| `vite.config.ts` | Vite bundler configuration |
| `tailwind.config.js` | Tailwind CSS configuration |
| `tsconfig.json` | TypeScript configuration |
| `.eslintrc.cjs` | ESLint configuration |
| `.prettierrc` | Prettier formatting |
| `src-tauri/Cargo.toml` | Rust dependencies |
| `src-tauri/tauri.conf.json` | Tauri app configuration |
| `python/pyproject.toml` | Python project configuration |

## Assets (`assets/`)

```
assets/
├── maps/                   # CS2 radar images
│   ├── de_dust2.png
│   ├── de_mirage.png
│   └── ...
└── icons/                  # App icons (various sizes)
```

## Build Outputs

| Directory | Contents |
|-----------|----------|
| `build/` | Frontend build output |
| `src-tauri/target/` | Rust build output |
| `python/dist/` | Python package/executable |

## Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Svelte Components | PascalCase | `DemoCard.svelte` |
| TypeScript files | camelCase | `demoStore.ts` |
| Rust files | snake_case | `demo_service.rs` |
| Python files | snake_case | `demo_parser.py` |
| SQL migrations | numbered | `001_initial.sql` |
| CSS classes | kebab-case | `demo-card` |

## Import Aliases

| Alias | Path |
|-------|------|
| `$lib` | `src/lib` |
| `$app` | SvelteKit internals |
