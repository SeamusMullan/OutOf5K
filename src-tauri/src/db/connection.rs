//! Database connection management

use crate::error::{AppError, AppResult};
use crate::models::{Demo, DemoStatus, DemoSummary, PlayerStats, PlayerStatsByMap, Settings};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::path::Path;
use std::str::FromStr;

/// Database connection wrapper
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    /// Create a new database connection
    pub fn new(path: &Path) -> AppResult<Self> {
        let db_url = format!("sqlite:{}?mode=rwc", path.display());
        
        // Use tokio runtime for initialization
        let pool = tokio::runtime::Runtime::new()
            .map_err(|e| AppError::Database(sqlx::Error::Configuration(e.to_string().into())))?
            .block_on(async {
                let options = SqliteConnectOptions::from_str(&db_url)?
                    .create_if_missing(true)
                    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                    .foreign_keys(true);

                SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect_with(options)
                    .await
            })?;

        let db = Self { pool };
        
        // Run migrations
        tokio::runtime::Runtime::new()
            .map_err(|e| AppError::Database(sqlx::Error::Configuration(e.to_string().into())))?
            .block_on(db.run_migrations())?;

        Ok(db)
    }

    /// Run database migrations
    async fn run_migrations(&self) -> AppResult<()> {
        sqlx::query(include_str!("migrations/001_initial.sql"))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Get the connection pool
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    // Demo operations

    /// Insert a new demo
    pub async fn insert_demo(&self, demo: &Demo) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO demos (id, file_path, file_hash, map_name, played_at, duration_ticks, tickrate, status, parsed_at, metadata, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&demo.id)
        .bind(&demo.file_path)
        .bind(&demo.file_hash)
        .bind(&demo.map_name)
        .bind(&demo.played_at)
        .bind(demo.duration_ticks)
        .bind(demo.tickrate)
        .bind(demo.status.to_string())
        .bind(&demo.parsed_at)
        .bind(&demo.metadata)
        .bind(&demo.created_at)
        .bind(&demo.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Get a demo by ID
    pub async fn get_demo(&self, id: &str) -> AppResult<Option<Demo>> {
        let row = sqlx::query_as::<_, DemoRow>(
            "SELECT * FROM demos WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| r.into()))
    }

    /// Get a demo by file hash
    pub async fn get_demo_by_hash(&self, hash: &str) -> AppResult<Option<Demo>> {
        let row = sqlx::query_as::<_, DemoRow>(
            "SELECT * FROM demos WHERE file_hash = ?"
        )
        .bind(hash)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| r.into()))
    }

    /// List demos with pagination
    pub async fn list_demos(&self, limit: i32, offset: i32) -> AppResult<Vec<DemoSummary>> {
        let rows = sqlx::query_as::<_, DemoSummaryRow>(
            r#"
            SELECT 
                d.id,
                d.map_name,
                d.played_at,
                d.status,
                COUNT(DISTINCT dp.player_id) as player_count,
                COALESCE(MAX(r.ct_score) + MAX(r.t_score), 0) as total_rounds
            FROM demos d
            LEFT JOIN demo_players dp ON d.id = dp.demo_id
            LEFT JOIN rounds r ON d.id = r.demo_id
            GROUP BY d.id
            ORDER BY d.played_at DESC NULLS LAST, d.created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Delete a demo
    pub async fn delete_demo(&self, id: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM demos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Stats operations

    /// Get player stats
    pub async fn get_player_stats(&self, player_id: &str) -> AppResult<Option<PlayerStats>> {
        let row = sqlx::query_as::<_, PlayerStatsRow>(
            r#"
            SELECT 
                pms.player_id,
                p.name as player_name,
                COUNT(DISTINCT pms.demo_id) as matches,
                SUM(pms.kills) as total_kills,
                SUM(pms.deaths) as total_deaths,
                SUM(pms.assists) as total_assists,
                COALESCE(AVG(pms.adr), 0) as avg_adr,
                COALESCE(AVG(pms.rating), 0) as avg_rating,
                COALESCE(AVG(pms.kast), 0) * 100 as avg_kast,
                CASE WHEN SUM(pms.kills) > 0 
                    THEN CAST(SUM(pms.headshots) AS REAL) / SUM(pms.kills) * 100 
                    ELSE 0 
                END as hs_percentage
            FROM player_match_stats pms
            JOIN players p ON pms.player_id = p.steam_id
            WHERE pms.player_id = ?
            GROUP BY pms.player_id
            "#
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| r.into()))
    }

    /// Get player stats by map
    pub async fn get_player_stats_by_map(&self, player_id: &str) -> AppResult<Vec<PlayerStatsByMap>> {
        let rows = sqlx::query_as::<_, PlayerStatsByMapRow>(
            r#"
            SELECT 
                d.map_name,
                COUNT(DISTINCT d.id) as matches,
                SUM(pms.kills) as kills,
                SUM(pms.deaths) as deaths,
                COALESCE(AVG(pms.rating), 0) as avg_rating,
                COALESCE(AVG(pms.adr), 0) as avg_adr
            FROM player_match_stats pms
            JOIN demos d ON pms.demo_id = d.id
            WHERE pms.player_id = ?
            GROUP BY d.map_name
            ORDER BY matches DESC
            "#
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    // Settings operations

    /// Get settings
    pub async fn get_settings(&self) -> AppResult<Option<Settings>> {
        let row = sqlx::query_as::<_, SettingsRow>(
            "SELECT * FROM settings WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| r.into()))
    }

    /// Update settings
    pub async fn update_settings(&self, settings: &Settings) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO settings (id, demo_directory, steam_id, theme, auto_parse, parse_positions, position_interval)
            VALUES (1, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&settings.demo_directory)
        .bind(&settings.steam_id)
        .bind(&settings.theme)
        .bind(settings.auto_parse)
        .bind(settings.parse_positions)
        .bind(settings.position_interval)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

// Helper row types for sqlx

#[derive(sqlx::FromRow)]
struct DemoRow {
    id: String,
    file_path: String,
    file_hash: String,
    map_name: String,
    played_at: Option<chrono::DateTime<chrono::Utc>>,
    duration_ticks: i64,
    tickrate: i32,
    status: String,
    parsed_at: Option<chrono::DateTime<chrono::Utc>>,
    metadata: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<DemoRow> for Demo {
    fn from(row: DemoRow) -> Self {
        Demo {
            id: row.id,
            file_path: row.file_path,
            file_hash: row.file_hash,
            map_name: row.map_name,
            played_at: row.played_at,
            duration_ticks: row.duration_ticks,
            tickrate: row.tickrate,
            status: match row.status.as_str() {
                "pending" => DemoStatus::Pending,
                "parsing" => DemoStatus::Parsing,
                "parsed" => DemoStatus::Parsed,
                "error" => DemoStatus::Error,
                _ => DemoStatus::Pending,
            },
            parsed_at: row.parsed_at,
            metadata: row.metadata.and_then(|s| serde_json::from_str(&s).ok()),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct DemoSummaryRow {
    id: String,
    map_name: String,
    played_at: Option<chrono::DateTime<chrono::Utc>>,
    status: String,
    player_count: i32,
    total_rounds: i32,
}

impl From<DemoSummaryRow> for DemoSummary {
    fn from(row: DemoSummaryRow) -> Self {
        DemoSummary {
            id: row.id,
            map_name: row.map_name,
            played_at: row.played_at,
            status: match row.status.as_str() {
                "pending" => DemoStatus::Pending,
                "parsing" => DemoStatus::Parsing,
                "parsed" => DemoStatus::Parsed,
                "error" => DemoStatus::Error,
                _ => DemoStatus::Pending,
            },
            player_count: row.player_count,
            total_rounds: row.total_rounds,
        }
    }
}

#[derive(sqlx::FromRow)]
struct PlayerStatsRow {
    player_id: String,
    player_name: String,
    matches: i32,
    total_kills: i32,
    total_deaths: i32,
    total_assists: i32,
    avg_adr: f64,
    avg_rating: f64,
    avg_kast: f64,
    hs_percentage: f64,
}

impl From<PlayerStatsRow> for PlayerStats {
    fn from(row: PlayerStatsRow) -> Self {
        PlayerStats {
            player_id: row.player_id,
            player_name: row.player_name,
            matches: row.matches,
            total_kills: row.total_kills,
            total_deaths: row.total_deaths,
            total_assists: row.total_assists,
            avg_adr: row.avg_adr,
            avg_rating: row.avg_rating,
            avg_kast: row.avg_kast,
            hs_percentage: row.hs_percentage,
        }
    }
}

#[derive(sqlx::FromRow)]
struct PlayerStatsByMapRow {
    map_name: String,
    matches: i32,
    kills: i32,
    deaths: i32,
    avg_rating: f64,
    avg_adr: f64,
}

impl From<PlayerStatsByMapRow> for PlayerStatsByMap {
    fn from(row: PlayerStatsByMapRow) -> Self {
        PlayerStatsByMap {
            map_name: row.map_name,
            matches: row.matches,
            kills: row.kills,
            deaths: row.deaths,
            avg_rating: row.avg_rating,
            avg_adr: row.avg_adr,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SettingsRow {
    #[allow(dead_code)]
    id: i32,
    demo_directory: Option<String>,
    steam_id: Option<String>,
    theme: String,
    auto_parse: bool,
    parse_positions: bool,
    position_interval: i32,
}

impl From<SettingsRow> for Settings {
    fn from(row: SettingsRow) -> Self {
        Settings {
            demo_directory: row.demo_directory,
            steam_id: row.steam_id,
            theme: row.theme,
            auto_parse: row.auto_parse,
            parse_positions: row.parse_positions,
            position_interval: row.position_interval,
        }
    }
}
