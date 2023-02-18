use super::game::GameDb;
use rusqlite::{Connection, Result};

struct Migrations {}

impl Migrations {
    pub fn migrate(conn: &Connection) -> Result<()> {
        create_game_table(conn)?;
        Ok(())
    }
}

fn create_game_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS game (
            game_id   INTEGER PRIMARY KEY,
            name      TEXT NOT NULL,
            cover     TEXT,
            setup     TEXT,
            hints     TEXT,
            year      TEXT,
            version   TEXT,
            status    TEXT,
            added     VARCHAR(10),
            updated   VARCHAR(10)
        )",
        (),
    )?;
    Ok(())
}

#[cfg(test)]
mod migrations_tests {
    use super::*;
    #[test]
    fn test_create_game_table() {
        let conn = Connection::open_in_memory().unwrap();
        create_game_table(&conn).unwrap();
        let mut game = GameDb::default();
        game.name = "Game 1".to_string();
        conn.execute(
            "INSERT INTO game (game_id, name) VALUES (?1, ?2)",
            (game.id, &game.name),
        )
        .unwrap();
    }
}
