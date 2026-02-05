# ADR-004: SQLite as Primary Database

## Status

Accepted

## Date

2024-01-15

## Context

OutOf5K needs persistent storage for:
- Parsed demo metadata and events (potentially millions of rows per demo)
- Player statistics and profiles
- User preferences and settings
- Cached external API data

As a desktop application with local-first design, the database choice must support:
- Zero-configuration setup
- Single-user access
- Portable data (easy backup/migration)
- Good query performance for analytical queries
- Reasonable storage efficiency

Options considered:
- **SQLite**: Embedded, file-based SQL database
- **PostgreSQL**: Full-featured relational database
- **MongoDB**: Document-oriented NoSQL database
- **DuckDB**: Analytical database optimized for OLAP

## Decision

We will use **SQLite** as our primary database, accessed via the **sqlx** library in Rust.

## Consequences

### Positive

1. **Zero Configuration**: SQLite requires no server setup, no background processes, no ports. Perfect for desktop applications.

2. **Single File**: All data stored in one file. Users can easily:
   - Back up their data (copy the file)
   - Migrate to a new machine
   - Reset by deleting the file

3. **Excellent Performance for Our Use Case**: 
   - Fast for single-user read-heavy workloads
   - WAL mode provides good concurrent read performance
   - Handles millions of rows efficiently with proper indexing

4. **Battle-Tested**: SQLite is the most widely deployed database engine in the world. It's reliable, well-documented, and bug-free for our use cases.

5. **SQL Support**: Full SQL support means complex analytical queries are possible:
   ```sql
   SELECT map_name, AVG(rating), COUNT(*) 
   FROM player_stats 
   GROUP BY map_name 
   ORDER BY AVG(rating) DESC;
   ```

6. **JSON Support**: SQLite 3.38+ has good JSON functions, allowing flexible schema for event data:
   ```sql
   SELECT JSON_EXTRACT(data, '$.weapon') as weapon
   FROM events WHERE event_type = 'KILL';
   ```

7. **Rust Integration**: sqlx provides compile-time checked queries and async support.

### Negative

1. **Single-Writer Limitation**: Only one process can write at a time. Not an issue for single-user desktop app, but relevant if we add cloud sync.

2. **No Built-in Replication**: If we add team features with cloud sync, we'll need a separate solution (probably a cloud PostgreSQL instance).

3. **Limited Full-Text Search**: SQLite FTS is functional but not as sophisticated as dedicated search engines. May need alternative for future search features.

4. **Large BLOB Handling**: Storing large binary data (like position snapshots) requires care. We'll use efficient serialization.

### Neutral

1. **Schema Migrations**: Need to implement our own migration system (or use existing library). This is common for any SQL database.

2. **Type System**: SQLite's type affinity is flexible but less strict than PostgreSQL. We enforce types at the application layer.

## Schema Highlights

### Performance Optimizations

```sql
-- Use WAL mode for better concurrent reads
PRAGMA journal_mode = WAL;

-- Reasonable cache size (adjust based on available memory)
PRAGMA cache_size = -64000; -- 64MB

-- Enable foreign keys
PRAGMA foreign_keys = ON;
```

### Indexing Strategy

```sql
-- Primary access patterns
CREATE INDEX idx_demos_played_at ON demos(played_at DESC);
CREATE INDEX idx_events_demo_type ON events(demo_id, event_type);
CREATE INDEX idx_events_round ON events(round_id);

-- JSON indexes for common queries (SQLite 3.38+)
CREATE INDEX idx_events_attacker ON events(
    demo_id, 
    JSON_EXTRACT(data, '$.attacker_id')
) WHERE event_type = 'KILL';
```

### Event Data Storage

Event-specific data stored as JSON for flexibility:

```sql
CREATE TABLE events (
    id INTEGER PRIMARY KEY,
    demo_id TEXT NOT NULL,
    round_id INTEGER NOT NULL,
    tick INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    data JSON NOT NULL  -- Event-specific payload
);

-- Example kill event data:
-- {"attacker_id": "123", "victim_id": "456", "weapon": "ak47", "headshot": true}
```

## Alternatives Considered

### Alternative 1: PostgreSQL

Full-featured relational database.

**Why Rejected:**
- Requires running a database server
- Overkill for single-user desktop application
- Complicates distribution and installation
- Could be used for future cloud features, but not for local storage

### Alternative 2: MongoDB

Document-oriented NoSQL database.

**Why Rejected:**
- Also requires a server process
- Less suitable for analytical queries (aggregations are complex)
- No clear advantage for our semi-structured event data (SQLite JSON works fine)

### Alternative 3: DuckDB

OLAP-optimized embedded database.

**Why Rejected:**
- Optimized for analytical queries but we have mixed OLTP/OLAP workload
- Younger ecosystem, less battle-tested
- Column-oriented storage less efficient for our access patterns
- Could be considered for future reporting/analytics features

### Alternative 4: IndexedDB (Web)

Browser-native storage.

**Why Rejected:**
- Size limits vary by browser
- No SQL support
- Would require significant effort to use from Tauri backend
- Less powerful for complex queries

## Future Considerations

### Cloud Sync (Phase 5)

When we implement team features, we'll likely:
1. Keep SQLite for local storage (offline-first)
2. Add cloud PostgreSQL for shared team data
3. Implement sync logic to merge local and cloud state

### Large Dataset Handling

If users accumulate thousands of demos:
1. Consider archiving old data
2. Implement data retention policies
3. Optimize queries with pagination
4. Consider DuckDB for heavy analytical workloads

## References

- [SQLite Documentation](https://sqlite.org/docs.html)
- [sqlx Rust Library](https://github.com/launchbadge/sqlx)
- [SQLite Performance Tuning](https://sqlite.org/optimization.html)
- [SQLite JSON Functions](https://sqlite.org/json1.html)
