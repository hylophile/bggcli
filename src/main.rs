use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use futures::TryStreamExt;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use indicatif::{ProgressBar, ProgressStyle};
use quick_xml::de::from_str;
use reqwest::{Client, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{
    default_on_request_failure, policies::ExponentialBackoff, RetryTransientMiddleware, Retryable,
    RetryableStrategy,
};
use sqlx::{
    prelude::FromRow, sqlite::SqliteConnectOptions, Pool, QueryBuilder, Sqlite, SqlitePool,
};
use tokio::time::{sleep, Duration};

mod boardgame;
mod collection;
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
    Db {
        #[command(subcommand)]
        subcommand: DbCommands,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// deletes the database
    Clear,
    /// prints the database path
    Path,
}

#[derive(FromRow)]
struct Bleh {
    #[sqlx(default)]
    id: Option<i64>,
    #[sqlx(default)]
    name: Option<String>,
}

enum BGGAPIURLType {
    Collection,
    Geeklist,
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

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(5);
    let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::ForceCache, // prefer local version TODO: make configurable
            // mode: CacheMode::Default,
            manager: CACacheManager {
                path: http_cache_dir,
            },
            options: HttpCacheOptions::default(),
        }))
        .with(RetryTransientMiddleware::new_with_policy_and_strategy(
            retry_policy,
            Retry202,
        ))
        .build();

    let cli = Cli::parse();
    match cli.command {
        Commands::Add { url } => {
            let (url_type, url_api) = parse_bgg_url(&url)?;
            let resp = client.get(url_api).send().await?.text().await?;

            let ids = match url_type {
                BGGAPIURLType::Collection => {
                    let collection = match from_str::<collection::Items>(&resp) {
                        Ok(r) => r,
                        Err(e) => panic!("{}\n\n\ndocument:\n{}", e.to_string(), resp),
                    };
                    collection
                        .item
                        .iter()
                        .map(|it| {
                            let object_type = &it.objecttype;
                            let subtype = &it.subtype;
                            let id = it.objectid;
                            assert_eq!(object_type, "thing");
                            assert_eq!(subtype, "boardgame");
                            id
                        })
                        .collect::<Vec<u32>>()
                }
                BGGAPIURLType::Geeklist => {
                    let geeklist = match from_str::<geeklist::Geeklist>(&resp) {
                        Ok(r) => r,
                        Err(e) => panic!("{}\n\n\ndocument:\n{}", e.to_string(), resp),
                    };

                    geeklist
                        .item
                        .iter()
                        .map(|it| {
                            let object_type = &it.object_type;
                            let subtype = &it.subtype;
                            let id = it.object_id;
                            assert_eq!(object_type, "thing");
                            assert_eq!(subtype, "boardgame");
                            id
                        })
                        .collect::<Vec<u32>>()
                }
            };
            let _ = fetch_boardgame_ids(client, pool, ids).await?;
        }
        Commands::Query { r#where, columns } => {
            let filter = r#where.unwrap_or("true".to_string());
            let mut qb: QueryBuilder<Sqlite> =
                QueryBuilder::new("select name from boardgame where ");
            qb.push(filter);
            let q = qb.build_query_as::<Bleh>();
            let mut rows = q.fetch(&pool);

            while let Some(row) = rows.try_next().await? {
                let name = row.name.unwrap_or("???".into());
                println!("{}", name);
            }
        }
        Commands::Db { subcommand } => match subcommand {
            DbCommands::Clear => {
                std::fs::remove_file(&db_file)?;
                println!("Deleted database at {}", db_file.display());
            }
            DbCommands::Path => {
                println!("{}", db_file.display());
            }
        },
    };

    Ok(())
}

// const SUPPORTED_TYPES: &[&str] = &["geeklist", "boardgame", "expansion", "family"];
const SUPPORTED_TYPES: &[&str] = &["geeklist", "collection"];

fn parse_bgg_url(url: &str) -> Result<(BGGAPIURLType, String)> {
    if !url.starts_with("https://boardgamegeek.com/") {
        return Err(anyhow!(
            "Invalid domain: Only 'boardgamegeek.com' URLs are supported"
        ));
    }

    let path = url.trim_start_matches("https://boardgamegeek.com/");
    match path.split('/').collect::<Vec<_>>().as_slice() {
        ["geeklist", id, ..] => {
            if id.chars().all(char::is_numeric) {
                return Ok((
                    BGGAPIURLType::Geeklist,
                    format!("https://boardgamegeek.com/xmlapi2/geeklist/{id}/"),
                ));
            }
        }
        ["collection", "user", username, ..] => {
            return Ok((
                BGGAPIURLType::Collection,
                format!("https://boardgamegeek.com/xmlapi2/collection?username={username}"),
            ));
        }
        [type_part, ..] => {
            return Err(anyhow!(
                "Unsupported type '{}'. Supported types: {:?}",
                type_part,
                SUPPORTED_TYPES
            ));
        }
        _ => {
            return Err(anyhow!("Unknown url given"));
        }
    }

    Err(anyhow!(
        "Invalid URL format: Expected format is 'https://boardgamegeek.com/{{type}}/{{id}}/...'"
    ))
}

async fn fetch_boardgame_ids(
    client: ClientWithMiddleware,
    pool: Pool<Sqlite>,
    ids: Vec<u32>,
) -> Result<()> {
    let progress_bar_fetch = ProgressBar::new(ids.len() as u64);
    progress_bar_fetch.set_style(
        ProgressStyle::with_template("{elapsed:>4} [{bar:40}] {percent}% {msg}")
            .unwrap()
            .progress_chars("-Co"),
    );

    println!("Fetching & adding {} items...", ids.len());

    for chunk in ids.chunks(20) {
        let mut boardgames: Vec<boardgame::Item> = Vec::with_capacity(ids.len());
        let ids = chunk
            .iter()
            .map(|id| id.to_string())
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
        let task_wait = tokio::spawn(async move {
            if !resp_was_cached {
                sleep(Duration::from_secs(1)).await;
            }
        });

        let tpool = pool.clone();
        let task_add = tokio::spawn(async move { db::boardgames_insert(&tpool, boardgames).await });

        let _ = tokio::try_join!(task_wait, task_add)?;
    }
    progress_bar_fetch.finish();
    Ok(())
}

struct Retry202;
impl RetryableStrategy for Retry202 {
    fn handle(&self, res: &reqwest_middleware::Result<reqwest::Response>) -> Option<Retryable> {
        match res {
            // retry if 202
            Ok(success) if success.status() == 202 => Some(Retryable::Transient),
            // otherwise do not retry a successful request
            Ok(_) => None,
            // but maybe retry a request failure
            Err(error) => default_on_request_failure(error),
        }
    }
}
