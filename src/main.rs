use std::fmt::Debug;

use anyhow::Result;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use indicatif::{ProgressBar, ProgressStyle};
use quick_xml::de::from_str;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};

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
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bggcli")?;
    let http_cache_dir = xdg_dirs.create_cache_directory("http-cache")?;
    let db_file = {
        let mut data_dir = xdg_dirs.get_data_home();
        data_dir.push("default.db");
        data_dir
    };
    let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::ForceCache, // prefer local version TODO: make configurable
            // mode: CacheMode::Default,
            manager: CACacheManager {
                path: http_cache_dir,
            },
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

    let progress_bar_fetch = ProgressBar::new(boardgame_ids.len() as u64);
    progress_bar_fetch.set_style(
        ProgressStyle::with_template("{elapsed:>4} [{bar:40}] {percent}% {msg}")
            .unwrap()
            .progress_chars("-Co"),
    );

    println!("Fetching {} items...", boardgame_ids.len());

    for chunk in boardgame_ids.chunks(20) {
        let ids = chunk
            .iter()
            .map(|(_, id)| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let game_url = format!("https://boardgamegeek.com/xmlapi2/thing?id={ids}&stats=1");
        let mut delay = 1;
        let mut resp_was_cached = false;
        for _ in 0..5 {
            let resp = client.get(&game_url).send().await?;

            if resp.status() != reqwest::StatusCode::OK {
                if resp.status() == reqwest::StatusCode::ACCEPTED {
                    progress_bar_fetch.set_message(format!(
                        "TODO was requested from BGG. Retrying in {delay} seconds."
                    ));
                    sleep(Duration::from_secs(delay)).await;
                    delay *= 2;
                    continue;
                } else {
                    progress_bar_fetch
                        .set_message(format!("Failed. Reason: Status {}", resp.status(),));
                }
            }

            let headers = resp.headers().clone();
            let resp_text = resp.text().await?;
            match from_str::<boardgame::ResponseItems>(&resp_text) {
                Ok(parsed) => {
                    if let (Some(x_cache), Some(x_cache_lookup)) =
                        (headers.get("x-cache"), headers.get("x-cache-lookup"))
                    {
                        resp_was_cached = x_cache == "HIT" && x_cache_lookup == "HIT"
                    }
                    progress_bar_fetch.inc(parsed.item.len() as u64);
                    for item in parsed.item {
                        boardgames.push(item.clone());
                    }
                    break;
                }
                Err(e) => {
                    progress_bar_fetch.set_message(format!(
                        "Parsing failed. Retrying in {delay} seconds. Reason: {e}\n{resp_text}"
                    ));
                    sleep(Duration::from_secs(delay)).await;
                    delay *= 2;
                }
            }
        }
        if !resp_was_cached {
            sleep(Duration::from_secs(1)).await;
        }
    }
    progress_bar_fetch.finish();

    println!("Adding {} items to database...", boardgame_ids.len());
    db::boardgames_insert(&pool, boardgames).await?;

    // connection.close()?;
    // and limiting the set of fields that are to be serialized
    Ok(())
}
