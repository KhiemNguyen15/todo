use std::path::PathBuf;

use rusqlite::{Connection, Result, params};

pub fn init_db(db_path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            task TEXT NOT NULL,
            done INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn add_task(conn: &Connection, task: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO todo (task, done) VALUES (?1, 0)",
        params![task],
    )?;
    Ok(())
}

pub fn list_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, task, done FROM todo")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, bool>(2)?,
        ))
    })?;

    for row in rows {
        let (id, task, done) = row?;
        let status = if done { "✓" } else { " " };
        println!("[{}] {} - {}", id, status, task);
    }

    Ok(())
}

pub fn mark_done(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE todo SET done = 1 WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn clear_tasks(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM todo WHERE done = 1", [])?;
    Ok(())
}
