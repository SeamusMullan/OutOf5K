-- Initial database schema for OutOf5K
-- Migration 001

-- Demo files
CREATE TABLE IF NOT EXISTS demos (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL UNIQUE,
    file_hash TEXT NOT NULL,
    map_name TEXT NOT NULL,
    played_at DATETIME,
    duration_ticks INTEGER NOT NULL,
    tickrate INTEGER NOT NULL DEFAULT 64,
    status TEXT NOT NULL DEFAULT 'pending',
    parsed_at DATETIME,
    metadata TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Players (cached across demos)
CREATE TABLE IF NOT EXISTS players (
    steam_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    avatar_url TEXT,
    is_local_user INTEGER DEFAULT 0,
    first_seen DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_seen DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Player participation in demos
CREATE TABLE IF NOT EXISTS demo_players (
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(steam_id),
    team TEXT NOT NULL,
    start_team TEXT NOT NULL,
    PRIMARY KEY (demo_id, player_id)
);

-- Rounds
CREATE TABLE IF NOT EXISTS rounds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    round_number INTEGER NOT NULL,
    winner TEXT NOT NULL,
    win_reason TEXT NOT NULL,
    start_tick INTEGER NOT NULL,
    end_tick INTEGER NOT NULL,
    ct_score INTEGER NOT NULL,
    t_score INTEGER NOT NULL,
    UNIQUE(demo_id, round_number)
);

-- Game events
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    round_id INTEGER NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
    tick INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    data TEXT NOT NULL
);

-- Per-round player stats
CREATE TABLE IF NOT EXISTS player_round_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL,
    round_id INTEGER NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(steam_id),
    kills INTEGER DEFAULT 0,
    deaths INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    damage INTEGER DEFAULT 0,
    headshots INTEGER DEFAULT 0,
    flash_assists INTEGER DEFAULT 0,
    UNIQUE(demo_id, round_id, player_id)
);

-- Per-match player stats (aggregated)
CREATE TABLE IF NOT EXISTS player_match_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(steam_id),
    kills INTEGER DEFAULT 0,
    deaths INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    total_damage INTEGER DEFAULT 0,
    headshots INTEGER DEFAULT 0,
    adr REAL,
    kast REAL,
    rating REAL,
    first_kills INTEGER DEFAULT 0,
    first_deaths INTEGER DEFAULT 0,
    clutches_won INTEGER DEFAULT 0,
    clutches_lost INTEGER DEFAULT 0,
    UNIQUE(demo_id, player_id)
);

-- Detected highlights
CREATE TABLE IF NOT EXISTS highlights (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    round_id INTEGER REFERENCES rounds(id),
    player_id TEXT NOT NULL REFERENCES players(steam_id),
    highlight_type TEXT NOT NULL,
    start_tick INTEGER NOT NULL,
    end_tick INTEGER NOT NULL,
    description TEXT
);

-- User settings
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY DEFAULT 1,
    demo_directory TEXT,
    steam_id TEXT,
    theme TEXT NOT NULL DEFAULT 'dark',
    auto_parse INTEGER NOT NULL DEFAULT 1,
    parse_positions INTEGER NOT NULL DEFAULT 1,
    position_interval INTEGER NOT NULL DEFAULT 16
);

-- Performance-critical indexes
CREATE INDEX IF NOT EXISTS idx_demos_map ON demos(map_name);
CREATE INDEX IF NOT EXISTS idx_demos_played_at ON demos(played_at DESC);
CREATE INDEX IF NOT EXISTS idx_demos_status ON demos(status);

CREATE INDEX IF NOT EXISTS idx_rounds_demo ON rounds(demo_id);

CREATE INDEX IF NOT EXISTS idx_events_demo_type ON events(demo_id, event_type);
CREATE INDEX IF NOT EXISTS idx_events_round ON events(round_id);
CREATE INDEX IF NOT EXISTS idx_events_tick ON events(demo_id, tick);

CREATE INDEX IF NOT EXISTS idx_player_round_stats_demo ON player_round_stats(demo_id);
CREATE INDEX IF NOT EXISTS idx_player_round_stats_player ON player_round_stats(player_id);

CREATE INDEX IF NOT EXISTS idx_player_match_stats_player ON player_match_stats(player_id);
CREATE INDEX IF NOT EXISTS idx_player_match_stats_demo ON player_match_stats(demo_id);

CREATE INDEX IF NOT EXISTS idx_highlights_demo ON highlights(demo_id);
CREATE INDEX IF NOT EXISTS idx_highlights_player ON highlights(player_id);

-- Migrations tracking
CREATE TABLE IF NOT EXISTS _migrations (
    version INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO _migrations (version, name) VALUES (1, 'initial');
