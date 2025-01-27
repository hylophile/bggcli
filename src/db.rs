use anyhow::Result;
use sqlx::{Pool, QueryBuilder, Sqlite};
// use rusqlite::Connection;

// pub async fn init(pool: &Pool<Sqlite>) -> Result<()> {
//     sqlx::query!("",).execute(pool).await?;
//     Ok(())
// }

pub async fn boardgames_insert(
    pool: &Pool<Sqlite>,
    boardgames: Vec<crate::boardgame::Item>,
) -> Result<()> {
    let mut item_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
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
) ",
    );
    for b in boardgames {
        dbg!(b.id);

        //         sqlx::query!(
        //             "
        // INSERT OR REPLACE INTO item (
        //     id,
        //     item_type,
        //     text,
        //     thumbnail,
        //     image,
        //     description,
        //     yearpublished,
        //     minplayers,
        //     maxplayers,
        //     playingtime,
        //     minplaytime,
        //     maxplaytime,
        //     minage
        // )
        //     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        //     ",
        item_query_builder
            .push_bind(b.id)
            .push_bind(b.item_type)
            .push_bind(b.text)
            .push_bind(b.thumbnail)
            .push_bind(b.image)
            .push_bind(b.description)
            .push_bind(b.yearpublished.value)
            .push_bind(b.minplayers.value)
            .push_bind(b.maxplayers.value)
            .push_bind(b.playingtime.value)
            .push_bind(b.minplaytime.value)
            .push_bind(b.maxplaytime.value)
            .push_bind(b.minage.value);
        // )
        // .execute(pool)
        // .await?;

        for l in b.link {
            sqlx::query!(
                "
                INSERT OR REPLACE INTO link (
                 id,
                 item_id,
                 link_type,
                 value,
                 inbound   
                )
                VALUES (?, ?, ?, ?, ?)
            ",
                l.id,
                b.id,
                l.link_type,
                l.value,
                l.inbound
            )
            .execute(pool)
            .await?;
        }

        for n in b.name {
            sqlx::query!(
                "
                    INSERT OR REPLACE INTO itemname (
                        item_id,
                        name_type,
                        sortindex,
                        value
                    ) 
                    VALUES (?, ?, ?, ?)
                ",
                b.id,
                n.name_type,
                n.sortindex,
                n.value
            )
            .execute(pool)
            .await?;
        }

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
    item_query_builder.build().execute(pool).await?;

    Ok(())
}
