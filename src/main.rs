use anyhow::Result;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use quick_xml::de::from_str;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

use serde_rusqlite::*;
// use std::fs;
// use std::io;

mod boardgame;
mod geeklist;

// fn main() -> io::Result<()> {
//     // // Read the XML data from the file
//     // let filename = "data.xml";
//     // let filename = "list.xml";

//     // let xml_data = fs::read_to_string(filename)?;

//     // // Parse the XML data
//     // // match from_str::<boardgame::Items>(&xml_data) {
//     // //     Ok(items) => {
//     // //         println!("{:#?}", items);
//     // //     }
//     // //     Err(e) => {
//     // //         eprintln!("Error parsing XML: {}", e);
//     // //     }
//     // // }
//     // match from_str::<geeklist::Geeklist>(&xml_data) {
//     //     Ok(items) => {
//     //         println!("{:#?}", items);
//     //     }
//     //     Err(e) => {
//     //         eprintln!("Error parsing XML: {}", e);
//     //     }
//     // }
//     // Ok(())

//     // let config_path = xdg_dirs
//     //     .place_config_file("config.ini")
//     //     .expect("cannot create configuration directory");
//     // let cache_dir = xdg_dirs.get_cache_home();
//     // dbg!(boardgame_cache_dir);
//     // let mut config_file = File::create(config_path)?;
//     // write!(&mut config_file, "configured = 1")?;
//     let xdg_dirs = xdg::BaseDirectories::with_prefix("bggcli")?;

//     let boardgame_cache_dir = xdg_dirs.create_cache_directory("boardgames")?;
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://boardgamegeek.com/xmlapi2/geeklist/303652/more-games-playable-with-the-everdeck?itemid=9184614";
    // let url = "https://developer.mozilla.org/en-US/docs/Web/HTTP/Caching";
    let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::ForceCache, // prefer local version
            manager: CACacheManager::default(),
            options: HttpCacheOptions::default(),
        }))
        .build();
    let resp = client.get(url).send().await?.text().await?;

    let geeklist = from_str::<geeklist::Geeklist>(&resp)?;

    let boardgame_ids = geeklist
        .item
        .iter()
        .map(|it| {
            let object_type = &it.object_type;
            let subtype = &it.subtype;
            let id = it.object_id;
            assert_eq!(object_type, "thing");
            assert_eq!(subtype, "boardgame");
            return (&it.object_name, id);
        })
        .collect::<Vec<(&String, u32)>>();

    let mut boardgames: Vec<boardgame::Item> = Vec::with_capacity(boardgame_ids.len());

    for (name, id) in boardgame_ids {
        // let game_url =
        //     format!("https://boardgamegeek.com/xmlapi2/{object_type}?type={subtype}&id={id}");
        // println!("{name}, {id}");
        let game_url = format!("https://boardgamegeek.com/xmlapi2/thing?type=boardgame&id={id}");
        let resp = client.get(game_url).send().await?.text().await?;
        let parsed = from_str::<boardgame::Items>(&resp)?;
        boardgames.push(parsed.item);
    }

    let connection = rusqlite::Connection::open("my.db")?;
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS boardgames (id INT, name TEXT, PRIMARY KEY (id));
         CREATE TABLE IF NOT EXISTS mechanics  (id INT, name TEXT, PRIMARY KEY (id));        
         CREATE TABLE IF NOT EXISTS boardgames_mechanics  (boardgame_id INT, mechanic_id INT, PRIMARY KEY (boardgame_id, mechanic_id));        
        ",
    )?;

    for b in boardgames {
        dbg!(b.primary_name());
        // dbg!(b.mechanics());
        connection.execute(
            "INSERT OR REPLACE INTO boardgames (id, name) VALUES (?1, ?2)",
            (b.id, b.primary_name()),
        )?;
        for m in b.mechanics() {
            connection.execute(
                "INSERT OR REPLACE INTO mechanics (id, name) VALUES (?1, ?2)",
                (m.id, m.name),
            )?;
            connection.execute(
                "INSERT OR REPLACE INTO boardgames_mechanics (boardgame_id, mechanic_id) VALUES (?1, ?2)",
                (b.id,m.id),
            )?;
        }
    }

    // connection.close()?;
    // and limiting the set of fields that are to be serialized
    Ok(())
}
// SELECT
//     bg.id AS boardgame_id,
//     bg.name AS boardgame_name,
//     m.id AS mechanic_id,
//     m.name AS mechanic_name
// FROM
//     boardgames bg
// JOIN
//     boardgames_mechanics bgm ON bg.id = bgm.boardgame_id
// JOIN
//     mechanics m ON bgm.mechanic_id = m.id;
// --------------------------------------------------------------------------------------
// SELECT
//     m.id AS mechanic_id,
//     m.name AS mechanic_name,
//     COUNT(*) AS occurrence_count
// FROM
//     boardgames_mechanics bgm
// JOIN
//     mechanics m ON bgm.mechanic_id = m.id
// GROUP BY
//     m.id, m.name
// ORDER BY
//     occurrence_count DESC;
