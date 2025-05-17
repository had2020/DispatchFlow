use rusqlite::{params, Connection, Result};

pub fn already_user(username: String) -> bool {
    let conn = Connection::open("database.db").unwrap();

    // create accounts table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    let mut statement = conn
        .prepare("SELECT COUNT(*) FROM accounts WHERE username = ?1")
        .unwrap();
    let count: i64 = statement
        .query_row(params![username], |row| row.get(0))
        .unwrap_or(0);

    count > 0
}

pub fn create_user(username: String, password: String) {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        (),
    )
    .unwrap();

    conn.execute(
        "INSERT INTO accounts (username, password) VALUES (?1, ?2)",
        params![username, password],
    )
    .unwrap();
}

pub fn is_user(username: String, password: String) -> bool {
    let conn = Connection::open("database.db").unwrap();

    // Create accounts table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    let mut statement = conn
        .prepare("SELECT COUNT(*) FROM accounts WHERE username = ?1 AND password = ?2")
        .unwrap();

    let count: i64 = statement
        .query_row(params![username, password], |row| row.get(0))
        .unwrap();

    count > 0
}
