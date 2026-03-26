//! 数据库模块

use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLite 错误: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("数据库未初始化")]
    NotInitialized,
    #[error("记录不存在")]
    NotFound,
    #[error("记录已存在")]
    AlreadyExists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mindmap {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub category: Option<String>,
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Option<i64>,
    pub mindmap_id: Option<i64>,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<i64>,
    pub conversation_id: i64,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkillRequest {
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSkillRequest {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationRequest {
    pub mindmap_id: Option<i64>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConversationRequest {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    pub conversation_id: i64,
    pub role: String,
    pub content: String,
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &PathBuf) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS mindmaps (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS skills (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                content TEXT NOT NULL,
                category TEXT,
                version TEXT DEFAULT '1.0.0',
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mindmap_id INTEGER,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (mindmap_id) REFERENCES mindmaps(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                conversation_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_skills_category ON skills(category);
            CREATE INDEX IF NOT EXISTS idx_conversations_mindmap ON conversations(mindmap_id);
            CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id);
            "
        )?;
        log::info!("数据库表结构初始化完成");
        Ok(())
    }

    // ============ Mindmap CRUD ============
    pub fn create_mindmap(&self, title: &str) -> Result<i64, DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO mindmaps (title, content) VALUES (?1, ?2)",
            params![title, "{}"],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_mindmaps(&self) -> Result<Vec<Mindmap>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM mindmaps ORDER BY updated_at DESC"
        )?;
        let mindmaps = stmt.query_map([], |row| {
            Ok(Mindmap {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        let result: Vec<Mindmap> = mindmaps.filter_map(|m| m.ok()).collect();
        Ok(result)
    }

    pub fn get_mindmap(&self, id: i64) -> Result<Mindmap, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM mindmaps WHERE id = ?1"
        )?;
        let mindmap = stmt.query_row([id], |row| {
            Ok(Mindmap {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        Ok(mindmap)
    }

    pub fn update_mindmap(&self, id: i64, title: &str, content: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "UPDATE mindmaps SET title = ?1, content = ?2, updated_at = datetime('now') WHERE id = ?3",
            params![title, content, id],
        )?;
        if rows == 0 {
            return Err(DbError::NotFound);
        }
        Ok(())
    }

    pub fn delete_mindmap(&self, id: i64) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM mindmaps WHERE id = ?1", [id])?;
        if rows == 0 {
            return Err(DbError::NotFound);
        }
        Ok(())
    }

    // ============ Skill CRUD ============
    pub fn create_skill(&self, req: &CreateSkillRequest) -> Result<i64, DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO skills (name, description, content, category) VALUES (?1, ?2, ?3, ?4)",
            params![req.name, req.description, req.content, req.category],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_skills(&self, category: Option<&str>) -> Result<Vec<Skill>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut result = Vec::new();

        if let Some(cat) = category {
            let mut stmt = conn.prepare(
                "SELECT id, name, description, content, category, version, created_at, updated_at FROM skills WHERE category = ?1 ORDER BY name"
            )?;
            let mut rows = stmt.query([cat])?;
            while let Some(row) = rows.next()? {
                result.push(Skill {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    description: row.get(2)?,
                    content: row.get(3)?,
                    category: row.get(4)?,
                    version: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                });
            }
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, name, description, content, category, version, created_at, updated_at FROM skills ORDER BY name"
            )?;
            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                result.push(Skill {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    description: row.get(2)?,
                    content: row.get(3)?,
                    category: row.get(4)?,
                    version: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                });
            }
        }
        Ok(result)
    }

    pub fn get_skill(&self, id: i64) -> Result<Skill, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, content, category, version, created_at, updated_at FROM skills WHERE id = ?1"
        )?;
        let skill = stmt.query_row([id], |row| {
            Ok(Skill {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
                category: row.get(4)?,
                version: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;
        Ok(skill)
    }

    pub fn update_skill(&self, req: &UpdateSkillRequest) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "UPDATE skills SET name = ?1, description = ?2, content = ?3, category = ?4, updated_at = datetime('now') WHERE id = ?5",
            params![req.name, req.description, req.content, req.category, req.id],
        )?;
        if rows == 0 {
            return Err(DbError::NotFound);
        }
        Ok(())
    }

    pub fn delete_skill(&self, id: i64) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM skills WHERE id = ?1", [id])?;
        if rows == 0 {
            return Err(DbError::NotFound);
        }
        Ok(())
    }

    // ============ Conversation CRUD ============
    pub fn create_conversation(&self, req: &CreateConversationRequest) -> Result<i64, DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO conversations (mindmap_id, title) VALUES (?1, ?2)",
            params![req.mindmap_id, req.title],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_conversations(&self, mindmap_id: Option<i64>) -> Result<Vec<Conversation>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut result = Vec::new();

        if let Some(id) = mindmap_id {
            let mut stmt = conn.prepare(
                "SELECT id, mindmap_id, title, created_at, updated_at FROM conversations WHERE mindmap_id = ?1 ORDER BY updated_at DESC"
            )?;
            let mut rows = stmt.query([id])?;
            while let Some(row) = rows.next()? {
                result.push(Conversation {
                    id: Some(row.get(0)?),
                    mindmap_id: row.get(1)?,
                    title: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                });
            }
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, mindmap_id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC"
            )?;
            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                result.push(Conversation {
                    id: Some(row.get(0)?),
                    mindmap_id: row.get(1)?,
                    title: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                });
            }
        }
        Ok(result)
    }

    pub fn get_conversation(&self, id: i64) -> Result<Conversation, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, mindmap_id, title, created_at, updated_at FROM conversations WHERE id = ?1"
        )?;
        let conversation = stmt.query_row([id], |row| {
            Ok(Conversation {
                id: Some(row.get(0)?),
                mindmap_id: row.get(1)?,
                title: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        Ok(conversation)
    }

    pub fn update_conversation(&self, req: &UpdateConversationRequest) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "UPDATE conversations SET title = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![req.title, req.id],
        )?;
        if rows == 0 {
            return Err(DbError::NotFound);
        }
        Ok(())
    }

    pub fn delete_conversation(&self, id: i64) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM conversations WHERE id = ?1", [id])?;
        if rows == 0 {
            return Err(DbError::NotFound);
        }
        Ok(())
    }

    // ============ Message CRUD ============
    pub fn create_message(&self, req: &CreateMessageRequest) -> Result<i64, DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO messages (conversation_id, role, content) VALUES (?1, ?2, ?3)",
            params![req.conversation_id, req.role, req.content],
        )?;
        let msg_id = conn.last_insert_rowid();
        conn.execute(
            "UPDATE conversations SET updated_at = datetime('now') WHERE id = ?1",
            [req.conversation_id],
        )?;
        Ok(msg_id)
    }

    pub fn get_messages(&self, conversation_id: i64) -> Result<Vec<Message>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, conversation_id, role, content, created_at FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC"
        )?;
        let messages = stmt.query_map([conversation_id], |row| {
            Ok(Message {
                id: Some(row.get(0)?),
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        let result: Vec<Message> = messages.filter_map(|m| m.ok()).collect();
        Ok(result)
    }

    pub fn update_message_content(&self, id: i64, content: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "UPDATE messages SET content = ?1 WHERE id = ?2",
            params![content, id],
        )?;
        if rows == 0 {
            return Err(DbError::NotFound);
        }
        Ok(())
    }

    pub fn delete_message(&self, id: i64) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM messages WHERE id = ?1", [id])?;
        if rows == 0 {
            return Err(DbError::NotFound);
        }
        Ok(())
    }
}
