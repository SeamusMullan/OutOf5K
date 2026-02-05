# Design Patterns Glossary

## Overview

This document describes the design patterns used throughout OutOf5K. Each pattern is documented with its purpose, where it's used, and implementation examples specific to this codebase.

## Patterns Index

| Pattern | Category | Primary Use |
|---------|----------|-------------|
| [Repository](#repository-pattern) | Structural | Database access abstraction |
| [Command](#command-pattern) | Behavioral | Tauri IPC interface |
| [Observer/Pub-Sub](#observer-pattern) | Behavioral | Event propagation, Svelte stores |
| [Factory](#factory-pattern) | Creational | Creating parser instances |
| [Strategy](#strategy-pattern) | Behavioral | Pluggable analysis algorithms |
| [Adapter](#adapter-pattern) | Structural | External API clients |
| [Facade](#facade-pattern) | Structural | Python service interface |
| [Builder](#builder-pattern) | Creational | Complex query construction |

---

## Repository Pattern

### Purpose

Abstracts data persistence, providing a collection-like interface for domain objects. Separates business logic from data access concerns.

### Where Used

- **Rust Backend**: All database operations go through repository structs
- **Provides**: Clean API for CRUD operations, query composition, transaction management

### Structure

```
┌─────────────────────┐     ┌─────────────────────┐
│    Demo Service     │     │   Stats Service     │
└──────────┬──────────┘     └──────────┬──────────┘
           │                           │
           ▼                           ▼
┌─────────────────────────────────────────────────┐
│              Repository Layer                    │
│  ┌──────────────┐  ┌──────────────┐            │
│  │DemoRepository│  │StatsRepository│  ...      │
│  └──────────────┘  └──────────────┘            │
└─────────────────────────────────────────────────┘
                       │
                       ▼
              ┌─────────────────┐
              │     SQLite      │
              └─────────────────┘
```

### Implementation

```rust
// src-tauri/src/db/repositories/demo_repository.rs

pub struct DemoRepository {
    pool: SqlitePool,
}

impl DemoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Find a demo by ID
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Demo>> {
        sqlx::query_as!(Demo, 
            "SELECT * FROM demos WHERE id = ?", 
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(Into::into)
    }
    
    /// List demos with pagination and filters
    pub async fn list(&self, filters: DemoFilters, pagination: Pagination) -> Result<Vec<Demo>> {
        let mut query = QueryBuilder::new("SELECT * FROM demos WHERE 1=1");
        
        if let Some(map) = &filters.map_name {
            query.push(" AND map_name = ").push_bind(map);
        }
        if let Some(status) = &filters.status {
            query.push(" AND status = ").push_bind(status);
        }
        if let Some(after) = filters.played_after {
            query.push(" AND played_at > ").push_bind(after);
        }
        
        query.push(" ORDER BY played_at DESC");
        query.push(" LIMIT ").push_bind(pagination.limit);
        query.push(" OFFSET ").push_bind(pagination.offset);
        
        query.build_query_as::<Demo>()
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }
    
    /// Save a new demo
    pub async fn save(&self, demo: &Demo) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO demos (id, file_path, file_hash, map_name, played_at, 
                              duration_ticks, tickrate, status, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            demo.id, demo.file_path, demo.file_hash, demo.map_name,
            demo.played_at, demo.duration_ticks, demo.tickrate, 
            demo.status, demo.metadata
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Update demo status
    pub async fn update_status(&self, id: &str, status: DemoStatus) -> Result<()> {
        sqlx::query!(
            "UPDATE demos SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            status, id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Delete demo and all related data (cascades)
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM demos WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
}
```

### Benefits

- **Testability**: Easy to mock for unit tests
- **Flexibility**: Change database implementation without affecting services
- **Consistency**: Centralized query logic and error handling

---

## Command Pattern

### Purpose

Encapsulates a request as an object, allowing parameterization, queuing, and logging of requests. In Tauri, commands are the primary IPC mechanism.

### Where Used

- **Tauri Backend**: All frontend-to-backend communication
- **Python Bridge**: Request/response protocol

### Structure

```
┌─────────────────┐      invoke("cmd", params)      ┌─────────────────┐
│    Frontend     │ ──────────────────────────────► │  Command        │
│    (Svelte)     │                                 │  Handler        │
│                 │ ◄────────────────────────────── │  (Rust)         │
└─────────────────┘        Result/Error             └─────────────────┘
```

### Implementation

```rust
// src-tauri/src/commands/demo_commands.rs

use tauri::command;

/// Import demo files
#[command]
pub async fn import_demos(
    state: State<'_, AppState>,
    paths: Vec<String>,
) -> Result<ImportResult, CommandError> {
    let demo_service = &state.demo_service;
    
    let results = demo_service.import_demos(&paths).await?;
    
    Ok(ImportResult {
        imported: results.iter().filter(|r| r.success).count(),
        failed: results.iter().filter(|r| !r.success).count(),
        details: results,
    })
}

/// Get demo by ID
#[command]
pub async fn get_demo(
    state: State<'_, AppState>,
    id: String,
) -> Result<Demo, CommandError> {
    let repo = &state.demo_repository;
    
    repo.find_by_id(&id)
        .await?
        .ok_or(CommandError::NotFound(format!("Demo {} not found", id)))
}

/// Get player statistics
#[command]
pub async fn get_player_stats(
    state: State<'_, AppState>,
    player_id: String,
    filters: Option<StatsFilters>,
) -> Result<PlayerStats, CommandError> {
    let stats_service = &state.stats_service;
    
    stats_service.get_player_stats(&player_id, filters).await
}

// Register commands in main.rs
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            import_demos,
            get_demo,
            get_player_stats,
            // ... more commands
        ])
        .run(tauri::generate_context!())
        .expect("error running app");
}
```

### Frontend Usage

```typescript
// src/lib/api/demos.ts
import { invoke } from '@tauri-apps/api/tauri';

export async function importDemos(paths: string[]): Promise<ImportResult> {
  return invoke('import_demos', { paths });
}

export async function getDemo(id: string): Promise<Demo> {
  return invoke('get_demo', { id });
}

export async function getPlayerStats(
  playerId: string, 
  filters?: StatsFilters
): Promise<PlayerStats> {
  return invoke('get_player_stats', { playerId, filters });
}
```

---

## Observer Pattern

### Purpose

Defines a one-to-many dependency between objects so that when one object changes state, all dependents are notified automatically.

### Where Used

- **Svelte Stores**: Reactive state management
- **Tauri Events**: Backend-to-frontend notifications
- **Python Progress**: Real-time progress updates

### Structure

```
┌─────────────┐     subscribe      ┌─────────────┐
│  Component  │ ◄───────────────── │   Store     │
│   A         │                    │  (Subject)  │
└─────────────┘                    └──────┬──────┘
                                          │ notify
┌─────────────┐     subscribe             │
│  Component  │ ◄─────────────────────────┤
│   B         │                           │
└─────────────┘                           │
                                          │
┌─────────────┐     subscribe             │
│  Component  │ ◄─────────────────────────┘
│   C         │
└─────────────┘
```

### Implementation: Svelte Stores

```typescript
// src/lib/stores/demos.ts
import { writable, derived } from 'svelte/store';

interface DemoState {
  demos: Demo[];
  loading: boolean;
  error: string | null;
  filters: DemoFilters;
}

function createDemoStore() {
  const { subscribe, set, update } = writable<DemoState>({
    demos: [],
    loading: false,
    error: null,
    filters: {},
  });

  return {
    subscribe,
    
    // Actions
    async loadDemos() {
      update(s => ({ ...s, loading: true, error: null }));
      try {
        const demos = await invoke<Demo[]>('list_demos', { filters: get(this).filters });
        update(s => ({ ...s, demos, loading: false }));
      } catch (e) {
        update(s => ({ ...s, error: e.message, loading: false }));
      }
    },
    
    setFilters(filters: DemoFilters) {
      update(s => ({ ...s, filters }));
      this.loadDemos();
    },
    
    addDemo(demo: Demo) {
      update(s => ({ ...s, demos: [demo, ...s.demos] }));
    },
    
    updateDemoStatus(id: string, status: string) {
      update(s => ({
        ...s,
        demos: s.demos.map(d => d.id === id ? { ...d, status } : d),
      }));
    },
  };
}

export const demoStore = createDemoStore();

// Derived store for filtered view
export const parsedDemos = derived(
  demoStore,
  $store => $store.demos.filter(d => d.status === 'parsed')
);
```

### Implementation: Tauri Events

```rust
// Backend: Emit events
use tauri::{AppHandle, Manager};

pub fn emit_progress(app: &AppHandle, demo_id: &str, progress: u32) {
    app.emit_all("demo:progress", DemoProgressPayload {
        demo_id: demo_id.to_string(),
        progress,
    }).unwrap();
}
```

```typescript
// Frontend: Listen to events
import { listen } from '@tauri-apps/api/event';

// In component or store initialization
const unlisten = await listen<DemoProgressPayload>('demo:progress', (event) => {
  const { demo_id, progress } = event.payload;
  progressStore.update(demo_id, progress);
});

// Cleanup
onDestroy(() => unlisten());
```

---

## Factory Pattern

### Purpose

Creates objects without specifying the exact class. Useful when the creation process is complex or when the type depends on runtime conditions.

### Where Used

- **Demo Parser**: Creating appropriate parser for demo version
- **API Clients**: Creating configured HTTP clients
- **Analyzers**: Creating analysis pipelines

### Implementation

```python
# python/src/parser_factory.py

from abc import ABC, abstractmethod
from pathlib import Path
from typing import Optional

class DemoParser(ABC):
    @abstractmethod
    def parse(self, progress_callback) -> ParsedDemo:
        pass

class CS2DemoParser(DemoParser):
    """Parser for CS2 (Source 2) demos"""
    def __init__(self, file_path: Path):
        self.file_path = file_path
        self._parser = demoparser2.DemoParser(str(file_path))
    
    def parse(self, progress_callback) -> ParsedDemo:
        # CS2-specific parsing logic
        ...

class LegacyCSGOParser(DemoParser):
    """Parser for legacy CSGO (Source 1) demos"""
    def __init__(self, file_path: Path):
        self.file_path = file_path
    
    def parse(self, progress_callback) -> ParsedDemo:
        # CSGO-specific parsing logic
        ...


class DemoParserFactory:
    """Factory for creating appropriate demo parser"""
    
    @staticmethod
    def create(file_path: Path) -> DemoParser:
        version = DemoParserFactory._detect_version(file_path)
        
        if version == DemoVersion.CS2:
            return CS2DemoParser(file_path)
        elif version == DemoVersion.CSGO:
            return LegacyCSGOParser(file_path)
        else:
            raise UnsupportedDemoError(f"Unsupported demo version: {version}")
    
    @staticmethod
    def _detect_version(file_path: Path) -> DemoVersion:
        with open(file_path, 'rb') as f:
            header = f.read(16)
            
        # Check magic bytes and protocol version
        if header[:8] == b'HL2DEMO\x00':
            protocol = int.from_bytes(header[8:12], 'little')
            if protocol >= 4:
                return DemoVersion.CS2
            else:
                return DemoVersion.CSGO
        
        raise InvalidDemoError("Not a valid demo file")


# Usage
parser = DemoParserFactory.create(Path("/path/to/demo.dem"))
result = parser.parse(progress_callback=update_progress)
```

---

## Strategy Pattern

### Purpose

Defines a family of algorithms, encapsulates each one, and makes them interchangeable. Lets the algorithm vary independently from clients that use it.

### Where Used

- **Analysis Algorithms**: Different analysis types (stats, highlights, positions)
- **Export Formats**: Different output formats for data
- **Comparison Methods**: Different ways to compare player performance

### Implementation

```python
# python/src/analyzers.py

from abc import ABC, abstractmethod
from typing import List
from dataclasses import dataclass

@dataclass
class AnalysisResult:
    analyzer_name: str
    data: dict

class Analyzer(ABC):
    """Strategy interface for demo analyzers"""
    
    @property
    @abstractmethod
    def name(self) -> str:
        pass
    
    @abstractmethod
    def analyze(self, parsed_demo: ParsedDemo) -> AnalysisResult:
        pass


class StatsAnalyzer(Analyzer):
    """Calculates player statistics"""
    
    @property
    def name(self) -> str:
        return "stats"
    
    def analyze(self, parsed_demo: ParsedDemo) -> AnalysisResult:
        stats = {}
        for player in parsed_demo.players:
            stats[player.steam_id] = self._calculate_player_stats(
                player, parsed_demo.events, parsed_demo.rounds
            )
        return AnalysisResult(self.name, {"player_stats": stats})


class HighlightAnalyzer(Analyzer):
    """Detects notable gameplay moments"""
    
    @property
    def name(self) -> str:
        return "highlights"
    
    def analyze(self, parsed_demo: ParsedDemo) -> AnalysisResult:
        highlights = []
        highlights.extend(self._detect_aces(parsed_demo))
        highlights.extend(self._detect_clutches(parsed_demo))
        highlights.extend(self._detect_multi_kills(parsed_demo))
        return AnalysisResult(self.name, {"highlights": highlights})


class PositionAnalyzer(Analyzer):
    """Analyzes positioning patterns"""
    
    @property
    def name(self) -> str:
        return "positions"
    
    def analyze(self, parsed_demo: ParsedDemo) -> AnalysisResult:
        heatmaps = {}
        for player in parsed_demo.players:
            heatmaps[player.steam_id] = self._generate_heatmap(
                player.steam_id, parsed_demo.positions
            )
        return AnalysisResult(self.name, {"heatmaps": heatmaps})


class AnalysisPipeline:
    """Context that uses analyzer strategies"""
    
    def __init__(self, analyzers: List[Analyzer] = None):
        self.analyzers = analyzers or []
    
    def add_analyzer(self, analyzer: Analyzer):
        self.analyzers.append(analyzer)
    
    def run(self, parsed_demo: ParsedDemo) -> dict:
        results = {}
        for analyzer in self.analyzers:
            result = analyzer.analyze(parsed_demo)
            results[result.analyzer_name] = result.data
        return results


# Usage
pipeline = AnalysisPipeline([
    StatsAnalyzer(),
    HighlightAnalyzer(),
    PositionAnalyzer(),
])

results = pipeline.run(parsed_demo)
```

---

## Adapter Pattern

### Purpose

Converts the interface of a class into another interface that clients expect. Allows classes with incompatible interfaces to work together.

### Where Used

- **External API Clients**: Adapting Steam, HLTV, Faceit APIs to internal interfaces
- **Data Format Conversion**: Converting between external and internal data models

### Implementation

```rust
// src-tauri/src/services/api_clients.rs

use async_trait::async_trait;

/// Internal interface for player statistics
#[async_trait]
pub trait PlayerStatsProvider {
    async fn get_player_stats(&self, player_id: &str) -> Result<PlayerStats>;
    async fn get_match_history(&self, player_id: &str, limit: u32) -> Result<Vec<Match>>;
}

/// Internal data model
pub struct PlayerStats {
    pub rating: f32,
    pub kd_ratio: f32,
    pub headshot_percentage: f32,
    pub maps_played: u32,
}

/// HLTV API adapter
pub struct HLTVAdapter {
    client: reqwest::Client,
    base_url: String,
}

#[async_trait]
impl PlayerStatsProvider for HLTVAdapter {
    async fn get_player_stats(&self, player_id: &str) -> Result<PlayerStats> {
        // Fetch from HLTV (returns HTML that needs parsing)
        let html = self.client
            .get(format!("{}/player/{}", self.base_url, player_id))
            .send()
            .await?
            .text()
            .await?;
        
        // Parse HTML and convert to our internal format
        let hltv_stats = parse_hltv_player_page(&html)?;
        
        // Adapt to internal format
        Ok(PlayerStats {
            rating: hltv_stats.rating_2_0,
            kd_ratio: hltv_stats.kills as f32 / hltv_stats.deaths as f32,
            headshot_percentage: hltv_stats.hs_percent,
            maps_played: hltv_stats.maps,
        })
    }
    
    async fn get_match_history(&self, player_id: &str, limit: u32) -> Result<Vec<Match>> {
        // Similar adaptation logic
        ...
    }
}

/// Faceit API adapter
pub struct FaceitAdapter {
    client: reqwest::Client,
    api_key: String,
}

#[async_trait]
impl PlayerStatsProvider for FaceitAdapter {
    async fn get_player_stats(&self, player_id: &str) -> Result<PlayerStats> {
        // Fetch from Faceit (returns JSON)
        let response: FaceitPlayerResponse = self.client
            .get(format!("https://open.faceit.com/data/v4/players/{}/stats/cs2", player_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json()
            .await?;
        
        // Adapt to internal format
        Ok(PlayerStats {
            rating: calculate_rating_from_faceit(&response),
            kd_ratio: response.lifetime.kd_ratio.parse()?,
            headshot_percentage: response.lifetime.headshot_percentage.parse()?,
            maps_played: response.lifetime.matches.parse()?,
        })
    }
    
    async fn get_match_history(&self, player_id: &str, limit: u32) -> Result<Vec<Match>> {
        ...
    }
}

/// Service uses the abstract interface
pub struct ComparisonService {
    providers: Vec<Box<dyn PlayerStatsProvider>>,
}

impl ComparisonService {
    pub async fn get_combined_stats(&self, player_id: &str) -> Result<PlayerStats> {
        // Try each provider, use first successful result
        for provider in &self.providers {
            if let Ok(stats) = provider.get_player_stats(player_id).await {
                return Ok(stats);
            }
        }
        Err(Error::NoProviderAvailable)
    }
}
```

---

## Facade Pattern

### Purpose

Provides a simplified interface to a complex subsystem. Reduces coupling between clients and the subsystem.

### Where Used

- **Python Bridge**: Single interface to all Python functionality
- **Demo Service**: Orchestrates multiple repositories and services

### Implementation

```rust
// src-tauri/src/services/python_bridge.rs

/// Facade for all Python service interactions
pub struct PythonBridge {
    process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    request_id: AtomicU64,
}

impl PythonBridge {
    /// Start the Python service
    pub async fn start() -> Result<Self> {
        let mut process = Command::new("python")
            .arg("-m")
            .arg("outof5k_parser")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        
        // Wait for ready signal
        // ... initialization logic
        
        Ok(Self { process, stdin, stdout, request_id: AtomicU64::new(0) })
    }
    
    // === High-level API (Facade) ===
    
    /// Parse a demo file completely
    pub async fn parse_demo(&mut self, path: &Path) -> Result<ParsedDemo> {
        self.send_command("parse", json!({
            "file_path": path.to_string_lossy(),
            "options": {
                "extract_positions": true,
                "position_interval": 16,
            }
        })).await
    }
    
    /// Get just the metadata (fast)
    pub async fn get_metadata(&mut self, path: &Path) -> Result<DemoMetadata> {
        self.send_command("get_metadata", json!({
            "file_path": path.to_string_lossy(),
        })).await
    }
    
    /// Run specific analyzers
    pub async fn analyze(&mut self, path: &Path, analyzers: &[&str]) -> Result<AnalysisResults> {
        self.send_command("analyze", json!({
            "file_path": path.to_string_lossy(),
            "analyzers": analyzers,
        })).await
    }
    
    /// Check if service is healthy
    pub async fn health_check(&mut self) -> Result<bool> {
        let response: HealthResponse = self.send_command("ping", json!({})).await?;
        Ok(response.status == "ok")
    }
    
    /// Graceful shutdown
    pub async fn shutdown(&mut self) -> Result<()> {
        self.send_command::<()>("shutdown", json!({})).await?;
        self.process.wait()?;
        Ok(())
    }
    
    // === Internal implementation ===
    
    async fn send_command<T: DeserializeOwned>(&mut self, cmd: &str, params: Value) -> Result<T> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let request = json!({
            "id": id.to_string(),
            "cmd": cmd,
            "params": params,
        });
        
        // Send request
        writeln!(self.stdin, "{}", request)?;
        self.stdin.flush()?;
        
        // Read response
        loop {
            let mut line = String::new();
            self.stdout.read_line(&mut line)?;
            let response: Response = serde_json::from_str(&line)?;
            
            if response.id != id.to_string() {
                continue; // Not our response
            }
            
            match response.status.as_str() {
                "success" => {
                    return Ok(serde_json::from_value(response.data.unwrap())?);
                }
                "error" => {
                    return Err(Error::Python(response.error.unwrap()));
                }
                "progress" => {
                    // Emit progress event, continue waiting
                }
            }
        }
    }
}
```

---

## Builder Pattern

### Purpose

Separates the construction of a complex object from its representation. Allows the same construction process to create different representations.

### Where Used

- **Query Building**: Constructing complex SQL queries
- **Configuration**: Building application configuration
- **Request Building**: Constructing API requests

### Implementation

```rust
// src-tauri/src/db/query_builder.rs

pub struct DemoQueryBuilder {
    base_query: String,
    conditions: Vec<String>,
    bindings: Vec<Value>,
    order_by: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl DemoQueryBuilder {
    pub fn new() -> Self {
        Self {
            base_query: "SELECT * FROM demos".to_string(),
            conditions: vec![],
            bindings: vec![],
            order_by: None,
            limit: None,
            offset: None,
        }
    }
    
    pub fn with_status(mut self, status: DemoStatus) -> Self {
        self.conditions.push("status = ?".to_string());
        self.bindings.push(status.to_string().into());
        self
    }
    
    pub fn with_map(mut self, map_name: &str) -> Self {
        self.conditions.push("map_name = ?".to_string());
        self.bindings.push(map_name.into());
        self
    }
    
    pub fn played_after(mut self, date: DateTime<Utc>) -> Self {
        self.conditions.push("played_at > ?".to_string());
        self.bindings.push(date.to_rfc3339().into());
        self
    }
    
    pub fn played_before(mut self, date: DateTime<Utc>) -> Self {
        self.conditions.push("played_at < ?".to_string());
        self.bindings.push(date.to_rfc3339().into());
        self
    }
    
    pub fn with_player(mut self, player_id: &str) -> Self {
        self.base_query = "SELECT d.* FROM demos d \
            JOIN demo_players dp ON d.id = dp.demo_id".to_string();
        self.conditions.push("dp.player_id = ?".to_string());
        self.bindings.push(player_id.into());
        self
    }
    
    pub fn order_by_played_at(mut self, descending: bool) -> Self {
        self.order_by = Some(format!(
            "played_at {}", 
            if descending { "DESC" } else { "ASC" }
        ));
        self
    }
    
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
    
    pub fn build(self) -> (String, Vec<Value>) {
        let mut query = self.base_query;
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        if let Some(order) = self.order_by {
            query.push_str(&format!(" ORDER BY {}", order));
        }
        
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        (query, self.bindings)
    }
}

// Usage
let (query, bindings) = DemoQueryBuilder::new()
    .with_status(DemoStatus::Parsed)
    .with_map("de_dust2")
    .played_after(week_ago)
    .with_player("76561198012345678")
    .order_by_played_at(true)
    .limit(20)
    .offset(0)
    .build();
```

---

## Summary

| Pattern | Key Benefit | When to Use |
|---------|-------------|-------------|
| **Repository** | Data access abstraction | Database operations |
| **Command** | Request encapsulation | API/IPC interfaces |
| **Observer** | Loose coupling | Event systems, reactive UI |
| **Factory** | Object creation abstraction | Complex or conditional instantiation |
| **Strategy** | Algorithm interchangeability | Multiple algorithms for same task |
| **Adapter** | Interface compatibility | External system integration |
| **Facade** | Simplified interface | Complex subsystem access |
| **Builder** | Complex object construction | Multi-step object creation |

## Related Documents

- [Architecture Overview](../architecture/overview.md)
- [UML Class Diagrams](../architecture/uml/class/)
- [Data Flow Documentation](../data-flow/)
