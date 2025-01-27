use anyhow::Result;
use sqlx::{Pool, Sqlite};
// use rusqlite::Connection;

pub async fn init(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query!(
        "
CREATE TABLE IF NOT EXISTS item (
    id INTEGER PRIMARY KEY,
    item_type TEXT NOT NULL,
    text TEXT,
    thumbnail TEXT NOT NULL,
    image TEXT NOT NULL,
    description TEXT NOT NULL,
    yearpublished INTEGER NOT NULL,
    minplayers INTEGER NOT NULL,
    maxplayers INTEGER NOT NULL,
    playingtime INTEGER NOT NULL,
    minplaytime INTEGER NOT NULL,
    maxplaytime INTEGER NOT NULL,
    minage INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS itemname (
    id INTEGER PRIMARY KEY,
    item_id INTEGER NOT NULL,
    name_type TEXT NOT NULL,
    sortindex TEXT NOT NULL,
    value TEXT NOT NULL,
    FOREIGN KEY (item_id) REFERENCES item(id)
);

CREATE TABLE IF NOT EXISTS poll (
    id INTEGER PRIMARY KEY,
    item_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    title TEXT NOT NULL,
    totalvotes INTEGER NOT NULL,
    text TEXT,
    FOREIGN KEY (item_id) REFERENCES item(id)
);

CREATE TABLE IF NOT EXISTS results (
    id INTEGER PRIMARY KEY,
    poll_id INTEGER NOT NULL,
    numplayers TEXT,
    text TEXT,
    FOREIGN KEY (poll_id) REFERENCES poll(id)
);

CREATE TABLE IF NOT EXISTS resultsresult (
    id INTEGER PRIMARY KEY,
    results_id INTEGER NOT NULL,
    value TEXT NOT NULL,
    numvotes TEXT NOT NULL,
    level TEXT,
    FOREIGN KEY (results_id) REFERENCES results(id)
);

CREATE TABLE IF NOT EXISTS link (
    id INTEGER PRIMARY KEY,
    item_id INTEGER NOT NULL,
    link_type TEXT NOT NULL,
    value TEXT NOT NULL,
    inbound TEXT,
    FOREIGN KEY (item_id) REFERENCES item(id)
);
        ",
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn boardgames_insert(
    pool: &Pool<Sqlite>,
    boardgames: Vec<crate::boardgame::Item>,
) -> Result<()> {
    for b in boardgames {
        sqlx::query!(
            "
INSERT OR REPLACE INTO item (
    id,
    item_type,
    text,
    thumbnail,
    image,
    description,
    yearpublished,
    minplayers,
    maxplayers,
    playingtime,
    minplaytime,
    maxplaytime,
    minage
)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    ",
            b.id,
            b.item_type,
            b.text,
            b.thumbnail,
            b.image,
            b.description,
            b.yearpublished.value,
            b.minplayers.value,
            b.maxplayers.value,
            b.playingtime.value,
            b.minplaytime.value,
            b.maxplaytime.value,
            b.minage.value
        )
        .execute(pool)
        .await?;
        //     name: "first name".into(),
        // };
        // connection
        //     .execute(
        //         "INSERT INTO example (id, name) VALUES (:id, :name)",
        //         to_params_named(&row1).unwrap().to_slice().as_slice(),
        //     )
        //     .unwrap();

        // dbg!(b.primary_name());
        // dbg!(b.mechanics());
        // conn.execute(
        //     "INSERT OR REPLACE INTO boardgames (id, name) VALUES (?1, ?2)",
        //     (b.id, b.primary_name()),
        // )?;
        // for m in b.mechanics() {
        //     conn.execute(
        //         "INSERT OR REPLACE INTO mechanics (id, name) VALUES (?1, ?2)",
        //         (m.id, m.name),
        //     )?;
        //     conn.execute(
        //         "INSERT OR REPLACE INTO boardgames_mechanics (boardgame_id, mechanic_id) VALUES (?1, ?2)",
        //         (b.id,m.id),
        //     )?;
        // }
    }
    Ok(())
}
