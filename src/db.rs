use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Result, OptionalExtension};

use crate::models::{CreateUser, User};

/// 数据库连接池类型
pub type DbPool = Pool<SqliteConnectionManager>;

/// 初始化数据库连接池
pub fn init_pool(database_url: &str) -> Result<DbPool, r2d2::Error> {
    let manager = SqliteConnectionManager::file(database_url);
    Pool::new(manager)
}

/// 初始化数据库表结构
pub fn init_database(pool: &DbPool) -> Result<()> {
    let conn = pool.get().map_err(|e| {
        rusqlite::Error::InvalidParameterName(format!("连接池错误: {}", e))
    })?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // 插入初始数据（如果表为空）
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
    
    if count == 0 {
        conn.execute(
            "INSERT INTO users (name, email) VALUES (?1, ?2)",
            ["张三", "zhangsan@example.com"],
        )?;
        
        conn.execute(
            "INSERT INTO users (name, email) VALUES (?1, ?2)",
            ["李四", "lisi@example.com"],
        )?;
        
        log::info!("✅ 已插入初始数据");
    }

    log::info!("✅ 数据库初始化完成");
    Ok(())
}

/// 获取所有用户
pub fn get_all_users(pool: &DbPool) -> Result<Vec<User>> {
    let conn = pool.get().map_err(|e| {
        rusqlite::Error::InvalidParameterName(format!("连接池错误: {}", e))
    })?;

    let mut stmt = conn.prepare("SELECT id, name, email FROM users ORDER BY id")?;
    
    let users = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(users)
}

/// 根据 ID 获取用户
pub fn get_user_by_id(pool: &DbPool, user_id: i64) -> Result<Option<User>> {
    let conn = pool.get().map_err(|e| {
        rusqlite::Error::InvalidParameterName(format!("连接池错误: {}", e))
    })?;

    let mut stmt = conn.prepare("SELECT id, name, email FROM users WHERE id = ?1")?;
    
    let user = stmt.query_row([user_id], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    }).optional()?;

    Ok(user)
}

/// 创建新用户
pub fn create_user(pool: &DbPool, user: &CreateUser) -> Result<User> {
    let conn = pool.get().map_err(|e| {
        rusqlite::Error::InvalidParameterName(format!("连接池错误: {}", e))
    })?;

    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        [&user.name, &user.email],
    )?;

    let user_id = conn.last_insert_rowid();

    Ok(User {
        id: user_id,
        name: user.name.clone(),
        email: user.email.clone(),
    })
}

/// 删除用户
pub fn delete_user(pool: &DbPool, user_id: i64) -> Result<bool> {
    let conn = pool.get().map_err(|e| {
        rusqlite::Error::InvalidParameterName(format!("连接池错误: {}", e))
    })?;

    let affected = conn.execute("DELETE FROM users WHERE id = ?1", [user_id])?;

    Ok(affected > 0)
}

/// 更新用户
pub fn update_user(pool: &DbPool, user_id: i64, user: &CreateUser) -> Result<Option<User>> {
    let conn = pool.get().map_err(|e| {
        rusqlite::Error::InvalidParameterName(format!("连接池错误: {}", e))
    })?;

    let affected = conn.execute(
        "UPDATE users SET name = ?1, email = ?2 WHERE id = ?3",
        rusqlite::params![&user.name, &user.email, user_id],
    )?;

    if affected > 0 {
        Ok(Some(User {
            id: user_id,
            name: user.name.clone(),
            email: user.email.clone(),
        }))
    } else {
        Ok(None)
    }
}

