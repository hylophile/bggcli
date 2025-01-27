use anyhow::Result;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use quick_xml::de::from_str;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use sqlx::SqlitePool;

// use serde_rusqlite::*;
mod boardgame;
mod db;
mod geeklist;

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

    let pool = SqlitePool::connect("./my.db").await?;
    // let conn = rusqlite::Connection::open("my.db")?;
    // db::init(&pool).await?;

    for (name, id) in boardgame_ids {
        // let game_url =
        //     format!("https://boardgamegeek.com/xmlapi2/{object_type}?type={subtype}&id={id}");
        // println!("{name}, {id}");
        let game_url = format!("https://boardgamegeek.com/xmlapi2/thing?type=boardgame&id={id}");
        let resp = client.get(game_url).send().await?.text().await?;
        let parsed = from_str::<boardgame::ResponseItems>(&resp)?;
        // boardgames.push(parsed.item[0].clone());
        if let Some(item) = parsed.item.get(0) {
            // item.update()?;
            // dbg!(item.id);
            boardgames.push(item.clone())
        }
    }

    db::boardgames_insert(&pool, boardgames).await?;

    // connection.close()?;
    // and limiting the set of fields that are to be serialized
    Ok(())
}
