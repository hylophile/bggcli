use anyhow::Result;
use sqlx::{Pool, QueryBuilder, Sqlite};

pub async fn boardgames_insert(
    pool: &Pool<Sqlite>,
    boardgames: Vec<crate::boardgame::Item>,
) -> Result<()> {
    let mut item_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT OR REPLACE INTO item (
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
                    minage,
                    usersrated, 
                    average, 
                    bayesaverage, 
                    stddev, 
                    median, 
                    owned, 
                    trading, 
                    wanting, 
                    wishing, 
                    numcomments, 
                    numweights, 
                    averageweight,
                    rank
               ) ",
    );
    let mut itemname_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT OR REPLACE INTO itemname (
                   item_id,
                   name_type,
                   sortindex,
                   value
               ) VALUES ",
    );
    let mut link_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT OR REPLACE INTO link (
                   id,
                   link_type,
                   value,
                   inbound   
               ) VALUES ",
    );
    let mut item_link_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT OR REPLACE INTO item_link (
                   item_id,
                   link_id
               ) VALUES ",
    );
    let mut poll_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT OR REPLACE INTO poll (
                   item_id,
                   name,
                   title,
                   totalvotes
               ) VALUES ",
    );
    let mut result_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT OR REPLACE INTO result (
                   poll_id,
                   poll_name,
                   numplayers,
                   value,
                   numvotes,
                   level
               ) VALUES ",
    );
    let mut pollresultsummary_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT OR REPLACE INTO pollsummaryresult (
                   item_id,
                   poll_name,
                   poll_title,
                   result_name,
                   result_value
               ) VALUES ",
    );

    // TODO: feels very dumb, but it works
    let mut is_first_link = true;
    let mut is_first_item_link = true;
    let mut is_first_itemname = true;
    let mut is_first_poll = true;
    let mut is_first_result = true;
    let mut is_first_pollresultsummary = true;

    item_query_builder.push_values(boardgames, |mut item_qb, item| {
        let stats = item.statistics.ratings;
        let rank = stats
            .ranks
            .rank
            .iter()
            .filter(|r| r.name == "boardgame")
            .collect::<Vec<_>>()
            .get(0)
            .map(|r| (&r.value).parse::<i64>().ok())
            .flatten();
        item_qb
            .push_bind(item.id)
            .push_bind(item.item_type)
            .push_bind(item.text)
            .push_bind(item.thumbnail)
            .push_bind(item.image)
            .push_bind(item.description)
            .push_bind(item.yearpublished.value)
            .push_bind(item.minplayers.value)
            .push_bind(item.maxplayers.value)
            .push_bind(item.playingtime.value)
            .push_bind(item.minplaytime.value)
            .push_bind(item.maxplaytime.value)
            .push_bind(item.minage.value)
            .push_bind(stats.usersrated.value)
            .push_bind(stats.average.value)
            .push_bind(stats.bayesaverage.value)
            .push_bind(stats.stddev.value)
            .push_bind(stats.median.value)
            .push_bind(stats.owned.value)
            .push_bind(stats.trading.value)
            .push_bind(stats.wanting.value)
            .push_bind(stats.wishing.value)
            .push_bind(stats.numcomments.value)
            .push_bind(stats.numweights.value)
            .push_bind(stats.averageweight.value)
            .push_bind(rank);

        // TODO: add ranks

        // TODO: manually building sql queries, in this day and age? well, it works
        for link in item.link.iter() {
            if !is_first_link {
                link_query_builder.push(", ");
            }
            link_query_builder.push("(");
            let mut separated = link_query_builder.separated(", ");
            separated
                .push_bind(link.id)
                .push_bind(link.link_type.clone())
                .push_bind(link.value.clone())
                .push_bind(link.inbound.clone());
            separated.push_unseparated(")");
            is_first_link = false;

            if !is_first_item_link {
                item_link_query_builder.push(", ");
            }
            item_link_query_builder.push("(");
            let mut separated = item_link_query_builder.separated(", ");
            separated.push_bind(item.id).push_bind(link.id);
            separated.push_unseparated(")");
            is_first_item_link = false;
        }

        for name in item.name.iter() {
            if !is_first_itemname {
                itemname_query_builder.push(", ");
            }
            itemname_query_builder.push("(");
            let mut separated = itemname_query_builder.separated(", ");
            separated
                .push_bind(item.id)
                .push_bind(name.name_type.clone())
                .push_bind(name.sortindex.clone())
                .push_bind(name.value.clone());
            separated.push_unseparated(")");
            is_first_itemname = false;
        }

        for poll in item.poll.iter() {
            if !is_first_poll {
                poll_query_builder.push(", ");
            }
            poll_query_builder.push("(");
            let mut separated = poll_query_builder.separated(", ");
            separated
                .push_bind(item.id)
                .push_bind(poll.name.clone())
                .push_bind(poll.title.clone())
                .push_bind(poll.totalvotes);
            separated.push_unseparated(")");
            is_first_poll = false;

            for results in &poll.results {
                let num_players = results
                    .numplayers
                    .clone()
                    .and_then(parse_incremented_integer);
                for result in &results.result {
                    let level = result.level.clone().and_then(parse_incremented_integer);
                    if !is_first_result {
                        result_query_builder.push(", ");
                    }
                    result_query_builder.push("(");
                    let mut separated = result_query_builder.separated(", ");
                    separated
                        .push_bind(item.id)
                        .push_bind(poll.name.clone())
                        .push_bind(num_players.clone())
                        .push_bind(result.value.clone())
                        .push_bind(result.numvotes)
                        .push_bind(level);
                    separated.push_unseparated(")");
                    is_first_result = false;
                }
            }

            let pollsummary_name = item.poll_summary.name.clone();
            let pollsummary_title = item.poll_summary.title.clone();
            for psr in item.poll_summary.result.clone().iter() {
                if !is_first_pollresultsummary {
                    pollresultsummary_query_builder.push(", ");
                }
                pollresultsummary_query_builder.push("(");
                let mut separated = pollresultsummary_query_builder.separated(", ");
                let result_value = extract_players(&psr.value);
                separated
                    .push_bind(item.id)
                    .push_bind(pollsummary_name.clone())
                    .push_bind(pollsummary_title.clone())
                    .push_bind(psr.name.clone())
                    .push_bind(result_value);
                separated.push_unseparated(")");
                is_first_pollresultsummary = false;
            }
        }
    });

    let q = item_query_builder.build();
    q.execute(pool).await?;
    let q = link_query_builder.build();
    q.execute(pool).await?;
    let q = item_link_query_builder.build();
    q.execute(pool).await?;
    let q = itemname_query_builder.build();
    q.execute(pool).await?;
    let q = poll_query_builder.build();
    q.execute(pool).await?;
    let q = result_query_builder.build();
    q.execute(pool).await?;
    let q = pollresultsummary_query_builder.build();
    q.execute(pool).await?;

    Ok(())
}
// fn parse_level(input: String) -> Result<String, std::num::ParseIntError> {
fn parse_incremented_integer(input: String) -> Option<String> {
    if let Some(stripped) = input.strip_suffix('+') {
        stripped
            .parse::<i64>()
            .map(|n| n + 1)
            .map(|n| n.to_string())
            .ok() // meh... result would be better?
    } else {
        return Some(input.to_string());
    }
}

fn extract_players(input: &str) -> Option<String> {
    let pattern = r".+with (.+) players$";
    let re = regex::Regex::new(pattern).unwrap();

    if let Some(captures) = re.captures(input) {
        let players = captures.get(1).unwrap().as_str();
        let re = regex::Regex::new(r"(\d+)–(\d+)").unwrap();
        if let Some(captures) = re.captures(players) {
            let bottom = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let top = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            Some(
                (bottom..=top)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            )
        } else {
            Some(players.to_string())
        }
    } else {
        None
    }
}
