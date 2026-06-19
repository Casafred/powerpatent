use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// SQLite 缓存管理器
pub struct CacheManager {
    conn: Mutex<Connection>,
}

impl CacheManager {
    /// 打开或创建缓存数据库
    pub fn open(db_path: &str) -> Result<Self> {
        let path = Path::new(db_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(db_path)
            .map_err(|e| anyhow::anyhow!("打开缓存数据库失败: {}", e))?;

        // 创建表
        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS cache (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id TEXT NOT NULL,
                patent_id TEXT NOT NULL,
                module_id TEXT NOT NULL,
                level TEXT NOT NULL DEFAULT 'full',
                output_json TEXT NOT NULL,
                model TEXT NOT NULL,
                provider TEXT NOT NULL,
                prompt_version TEXT DEFAULT '1',
                temperature REAL DEFAULT 0.3,
                rerun_count INTEGER DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                UNIQUE(project_id, patent_id, module_id, level)
            );

            CREATE INDEX IF NOT EXISTS idx_cache_project ON cache(project_id);
            CREATE INDEX IF NOT EXISTS idx_cache_patent ON cache(project_id, patent_id);
        ").map_err(|e| anyhow::anyhow!("创建缓存表失败: {}", e))?;

        Ok(Self { conn: Mutex::new(conn) })
    }

    /// 查询缓存
    pub fn get(
        &self,
        project_id: &str,
        patent_id: &str,
        module_id: &str,
        level: &str,
    ) -> Result<Option<CacheEntry>> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("获取锁失败: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT project_id, patent_id, module_id, level, output_json, model, provider, prompt_version, temperature, rerun_count, created_at, updated_at FROM cache WHERE project_id = ?1 AND patent_id = ?2 AND module_id = ?3 AND level = ?4"
        ).map_err(|e| anyhow::anyhow!("准备查询失败: {}", e))?;

        let result = stmt.query_row(
            params![project_id, patent_id, module_id, level],
            |row| {
                Ok(CacheEntry {
                    project_id: row.get(0)?,
                    patent_id: row.get(1)?,
                    module_id: row.get(2)?,
                    level: row.get(3)?,
                    output_json: row.get(4)?,
                    model: row.get(5)?,
                    provider: row.get(6)?,
                    prompt_version: row.get(7)?,
                    temperature: row.get(8)?,
                    rerun_count: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            },
        );

        match result {
            Ok(entry) => Ok(Some(entry)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("查询缓存失败: {}", e)),
        }
    }

    /// 写入或更新缓存
    pub fn upsert(&self, entry: &CacheEntry) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("获取锁失败: {}", e))?;

        conn.execute(
            "INSERT INTO cache (project_id, patent_id, module_id, level, output_json, model, provider, prompt_version, temperature, rerun_count, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, datetime('now'), datetime('now'))
             ON CONFLICT(project_id, patent_id, module_id, level)
             DO UPDATE SET output_json = ?5, model = ?6, provider = ?7, prompt_version = ?8, temperature = ?9, rerun_count = rerun_count + 1, updated_at = datetime('now')",
            params![
                entry.project_id,
                entry.patent_id,
                entry.module_id,
                entry.level,
                entry.output_json,
                entry.model,
                entry.provider,
                entry.prompt_version,
                entry.temperature,
                entry.rerun_count,
            ],
        ).map_err(|e| anyhow::anyhow!("写入缓存失败: {}", e))?;

        Ok(())
    }

    /// 删除指定缓存
    pub fn delete(
        &self,
        project_id: &str,
        patent_id: &str,
        module_id: &str,
        level: &str,
    ) -> Result<bool> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("获取锁失败: {}", e))?;

        let affected = conn.execute(
            "DELETE FROM cache WHERE project_id = ?1 AND patent_id = ?2 AND module_id = ?3 AND level = ?4",
            params![project_id, patent_id, module_id, level],
        ).map_err(|e| anyhow::anyhow!("删除缓存失败: {}", e))?;

        Ok(affected > 0)
    }

    /// 清空项目缓存
    pub fn clear_project(&self, project_id: &str) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("获取锁失败: {}", e))?;

        conn.execute(
            "DELETE FROM cache WHERE project_id = ?1",
            params![project_id],
        ).map_err(|e| anyhow::anyhow!("清空项目缓存失败: {}", e))?;

        Ok(())
    }

    /// 获取项目所有缓存
    pub fn list_project(&self, project_id: &str) -> Result<Vec<CacheEntry>> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("获取锁失败: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT project_id, patent_id, module_id, level, output_json, model, provider, prompt_version, temperature, rerun_count, created_at, updated_at FROM cache WHERE project_id = ?1 ORDER BY patent_id, module_id"
        ).map_err(|e| anyhow::anyhow!("准备查询失败: {}", e))?;

        let entries = stmt.query_map(params![project_id], |row| {
            Ok(CacheEntry {
                project_id: row.get(0)?,
                patent_id: row.get(1)?,
                module_id: row.get(2)?,
                level: row.get(3)?,
                output_json: row.get(4)?,
                model: row.get(5)?,
                provider: row.get(6)?,
                prompt_version: row.get(7)?,
                temperature: row.get(8)?,
                rerun_count: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        }).map_err(|e| anyhow::anyhow!("查询缓存列表失败: {}", e))?
        .filter_map(|e| e.ok())
        .collect();

        Ok(entries)
    }
}

/// 缓存条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheEntry {
    pub project_id: String,
    pub patent_id: String,
    pub module_id: String,
    pub level: String,
    pub output_json: String,
    pub model: String,
    pub provider: String,
    pub prompt_version: String,
    pub temperature: f32,
    pub rerun_count: i32,
    pub created_at: String,
    pub updated_at: String,
}
