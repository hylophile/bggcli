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
                    averageweight
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

    // TODO: feels very dumb, but it works
    let mut is_first_link = true;
    let mut is_first_item_link = true;
    let mut is_first_itemname = true;

    item_query_builder.push_values(boardgames, |mut item_qb, item| {
        let stats = item.statistics.ratings;
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
            .push_bind(stats.averageweight.value);

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
    });

    let q = item_query_builder.build();
    q.execute(pool).await?;
    let q = link_query_builder.build();
    q.execute(pool).await?;
    let q = item_link_query_builder.build();
    q.execute(pool).await?;
    let q = itemname_query_builder.build();
    q.execute(pool).await?;

    Ok(())
}
