use anyhow::{anyhow, Result};
use boardgame::Item;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use indicatif::{ProgressBar, ProgressStyle};
use quick_xml::de::from_str;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use sqlx::{prelude::FromRow, sqlite::SqliteConnectOptions, QueryBuilder, Row, Sqlite, SqlitePool};
use tokio::time::{sleep, Duration};

use clap::{Parser, Subcommand};
use futures::TryStreamExt;

mod boardgame;
mod db;
mod geeklist;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    // name: Option<String>,

    /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// adds items to the database
    Add {
        /// URL to add
        url: String,
    },
    /// queries the database for items (SQL syntax)
    Query {
        /// add a WHERE-clause (Filter the results)
        #[arg(short, long)]
        r#where: Option<String>,
        /// declare which columns to show
        #[arg(short, long)]
        columns: Option<String>,
    },
}

#[derive(FromRow)]
struct Bleh {
    #[sqlx(default)]
    id: Option<i64>,
    #[sqlx(default)]
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bggcli")?;
    let http_cache_dir = xdg_dirs.create_cache_directory("http-cache")?;
    let db_file = xdg_dirs.place_data_file("default.db")?;
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(db_file.to_str().expect("unknown error"))
            .create_if_missing(true),
    )
    .await?;
    sqlx::migrate!().run(&pool).await?;

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

    let cli = Cli::parse();
    match cli.command {
        Commands::Add { url } => {
            let (url_type, id) = parse_bgg_url(&url)?;
            let url_api = format!("https://boardgamegeek.com/xmlapi2/{url_type}/{id}/");
            let boardgames = fetch_geeklist(client, &url_api).await?;
            println!("Adding {} items to database...", boardgames.len());
            db::boardgames_insert(&pool, boardgames).await?;
        }
        Commands::Query { r#where, columns } => {
            let filter = r#where.unwrap_or("mechanics like '%trick-taking%'".to_string());
            // let mut rows = sqlx::query("select name from boardgame limit 10").fetch(&pool);

            // while let Some(row) = rows.try_next().await? {
            //     // map the row into a user-defined domain type
            //     let name: &str = row.try_get("name")?;
            //     println!("{name}");
            // }

            let mut qb: QueryBuilder<Sqlite> =
                QueryBuilder::new("select name from boardgame where ");
            qb.push(filter);
            let q = qb.build_query_as::<Bleh>();
            let mut r = q.fetch(&pool);

            while let Some(row) = r.try_next().await? {
                let x = row.name.unwrap_or("???".into());
                println!("{x:?}");
            }
        }
    };

    Ok(())
}

// const SUPPORTED_TYPES: &[&str] = &["geeklist", "boardgame", "expansion", "family"];
const SUPPORTED_TYPES: &[&str] = &["geeklist"];

fn parse_bgg_url(url: &str) -> Result<(String, String)> {
    if !url.starts_with("https://boardgamegeek.com/") {
        return Err(anyhow!(
            "Invalid domain: Only 'boardgamegeek.com' URLs are supported"
        ));
    }

    let path = url.trim_start_matches("https://boardgamegeek.com/");
    let mut parts = path.split('/');
    if let (Some(type_part), Some(id_part)) = (parts.next(), parts.next()) {
        if !SUPPORTED_TYPES.contains(&type_part) {
            return Err(anyhow!(
                "Unsupported type '{}'. Supported types: {:?}",
                type_part,
                SUPPORTED_TYPES
            ));
        }
        if id_part.chars().all(char::is_numeric) {
            return Ok((type_part.to_string(), id_part.to_string()));
        }
    }

    Err(anyhow!(
        "Invalid URL format: Expected format is 'https://boardgamegeek.com/{{type}}/{{id}}/...'"
    ))
}

async fn fetch_geeklist(client: ClientWithMiddleware, url: &str) -> Result<Vec<Item>> {
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

    return Ok(boardgames);
}
