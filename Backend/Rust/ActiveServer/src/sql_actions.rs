use rand::prelude::*;
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

pub fn clear_user(username: String) -> bool {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "DELETE FROM accounts WHERE username = ?1",
        params![username],
    )
    .unwrap();

    true
}

pub fn create_team(username: String, team_name: String) -> String {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS teams (
            username TEXT NOT NULL,
            team_name TEXT NOT NULL,
            code TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    let mut statement = conn
        .prepare("SELECT COUNT(*) FROM teams WHERE username = ?1")
        .unwrap();
    let count: i64 = statement
        .query_row(params![username], |row| row.get(0))
        .unwrap_or(0);

    if count == 0 {
        // pusdeo random number gen
        let mut code: String = "".to_string();

        for i in 0..6 {
            let number = rand::random_range(0..9);
            code.push_str(&number.to_string());
        }

        conn.execute(
            "INSERT INTO teams (username, team_name, code) VALUES (?1, ?2, ?3)",
            params![username, team_name, code],
        )
        .unwrap();

        code
    } else {
        "Already".to_string()
    }
}

pub fn join_team(username: String, password: String, code: String) -> String {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS teams (
            username TEXT NOT NULL,
            team_name TEXT NOT NULL,
            code TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    let mut statement = conn
        .prepare("SELECT team_name FROM teams WHERE code = ?1")
        .unwrap();

    statement
        .query_row(params![code], |row| row.get(0))
        .unwrap_or_default()
}

pub fn set_duty_status(username: String, team_name: String, status: String) {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_status (
            username TEXT NOT NULL,
            team_name TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "DELETE FROM user_status WHERE username = ?1",
        params![username],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO user_status (username, team_name, status) VALUES (?1, ?2, ?3)",
        params![username, team_name, status],
    )
    .unwrap();
}

pub fn check_duty_status(username: String) -> String {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_status (
            username TEXT NOT NULL,
            team_name TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    let mut statement = conn
        .prepare("SELECT username FROM user_status WHERE status = ?1")
        .unwrap();

    statement
        .query_row(params![username], |row| row.get(0))
        .unwrap_or_default()
}

pub fn check_all_team_status(team_name: String) -> String {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_status (
            username TEXT NOT NULL,
            team_name TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    let mut statement = conn
        .prepare("SELECT username FROM user_status WHERE team_name = ?1")
        .unwrap();

    let usernames: Vec<String> = statement
        .query_map([team_name], |row| row.get(0))
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    usernames.join(", ")
}
