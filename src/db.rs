use std::path::PathBuf;

use rusqlite::{Connection, Result, params};
use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    pub idx: i32,
    pub task: String,
    pub done: bool,
    pub due: Option<String>,
}

pub fn init_db(db_path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            task TEXT NOT NULL,
            done INTEGER NOT NULL,
            due TEXT
        )",
        [],
    )?;

    Ok(conn)
}

pub fn add_task(conn: &Connection, task: &str, due: Option<&str>) -> Result<()> {
    conn.execute(
        "INSERT INTO todo (task, done, due) VALUES (?1, 0, ?2)",
        params![task, due],
    )?;
    Ok(())
}

pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT task, done, due FROM todo 
            ORDER BY due IS NULL, datetime(due)",
    )?;

    let row_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>("task")?,
            row.get::<_, bool>("done")?,
            row.get::<_, Option<String>>("due")?,
        ))
    })?;

    let results = row_iter
        .enumerate()
        .map(|(i, row_result)| {
            row_result.map(|(task, done, due)| Task {
                idx: (i + 1) as i32,
                task,
                done,
                due,
            })
        })
        .collect::<Result<Vec<Task>>>()?;

    Ok(results)
}

pub fn mark_done(conn: &Connection, idx: usize) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id FROM todo 
            ORDER BY due IS NULL, datetime(due) 
            LIMIT 1 OFFSET ?1",
    )?;

    let mut rows = stmt.query(params![idx as i64])?;

    if let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;

        conn.execute("UPDATE todo SET done = 1 WHERE id = ?1", params![id])?;

        Ok(())
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn remove_task(conn: &Connection, idx: usize) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id FROM todo 
            ORDER BY due IS NULL, datetime(due) 
            LIMIT 1 OFFSET ?1",
    )?;

    let mut rows = stmt.query(params![idx as i64])?;

    if let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;

        conn.execute("DELETE FROM todo WHERE id = ?1", params![id])?;

        Ok(())
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn clear_tasks(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM todo WHERE done = 1", [])?;
    Ok(())
}
