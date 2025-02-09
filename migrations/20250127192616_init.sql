-- Add migration script here

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
    minage INTEGER NOT NULL,
    usersrated INTEGER NOT NULL,
    average REAL NOT NULL,
    bayesaverage REAL NOT NULL,
    stddev REAL NOT NULL,
    median INTEGER NOT NULL,
    owned INTEGER NOT NULL,
    trading INTEGER NOT NULL,
    wanting INTEGER NOT NULL,
    wishing INTEGER NOT NULL,
    numcomments INTEGER NOT NULL,
    numweights INTEGER NOT NULL,
    averageweight REAL NOT NULL,
    rank INTEGER
);

CREATE TABLE IF NOT EXISTS itemname (
    item_id INTEGER NOT NULL,
    name_type TEXT NOT NULL,
    sortindex TEXT NOT NULL,
    value TEXT NOT NULL,
    FOREIGN KEY (item_id) REFERENCES item(id),
    PRIMARY KEY (item_id, value, name_type)
);

CREATE TABLE IF NOT EXISTS poll (
    item_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    title TEXT NOT NULL,
    totalvotes INTEGER NOT NULL,
    PRIMARY KEY (item_id, name),
    FOREIGN KEY (item_id) REFERENCES item(id)
);

CREATE TABLE IF NOT EXISTS result (
    poll_id INTEGER NOT NULL,
    poll_name TEXT NOT NULL,
    numplayers TEXT,
    value TEXT NOT NULL,
    numvotes INTEGER NOT NULL,
    level TEXT,
    PRIMARY KEY (poll_id, poll_name, numplayers, value, level),
    FOREIGN KEY (poll_id, poll_name) REFERENCES poll(item_id, name)
);

CREATE TABLE IF NOT EXISTS pollsummaryresult (
    item_id INTEGER NOT NULL,
    poll_name TEXT NOT NULL,
    poll_title TEXT NOT NULL,
    result_name TEXT NOT NULL,
    result_value TEXT,
    PRIMARY KEY (item_id, poll_name, result_name),
    FOREIGN KEY (item_id) REFERENCES item(id)
);

CREATE TABLE IF NOT EXISTS link (
    id INTEGER PRIMARY KEY,
    link_type TEXT NOT NULL,
    value TEXT NOT NULL,
    inbound TEXT
);

CREATE TABLE if not exists item_link (
    item_id integer,
    link_id integer,
    primary key (item_id, link_id),
    foreign key (item_id) references item(id),
    foreign key (link_id) references link(id)
);

create table if not exists rank (
    id integer,
    rank_type text not null,
    name text not null,  
    friendlyname text not null,
    primary key (id)
);

create table if not exists item_rank (
    item_id integer,
    rank_id integer,
    value integer,
    bayesavagerage real,
    primary key (item_id, rank_id),  
    foreign key (item_id) references item(id),
    foreign key (rank_id) references rank(id)
);

create view if not exists itemmechanic as
select item_link.item_id, group_concat(link.value, ', ') as value from item_link
join link on link.id == item_link.link_id 
where link.link_type="boardgamemechanic"
group by item_link.item_id;

create view if not exists itemfamily as
select item_link.item_id, group_concat(link.value, ', ') as value from item_link
join link on link.id == item_link.link_id 
where link.link_type="boardgamefamily"
group by item_link.item_id;

create view if not exists itemcategory as
select item_link.item_id, group_concat(link.value, ', ') as value from item_link
join link on link.id == item_link.link_id 
where link.link_type="boardgamecategory"
group by item_link.item_id;

-- not used anymore
-- create view if not exists _result_best_numplayers as
-- select r.poll_id, numplayers, max(r.numvotes) as numvotes from result r where r.poll_name ='suggested_numplayers' and r.value = 'Best' group by poll_id;


create view if not exists poll_bestwith as
select item_id, result_value from pollsummaryresult psr where psr.result_name = 'bestwith';

create view if not exists poll_recommendedwith as
select item_id, result_value from pollsummaryresult psr where psr.result_name = 'recommmendedwith';

create view if not exists boardgame as
select
    item.id,
    itemname.value as name,
    item.description,
    item.rank,
    item.averageweight,
    item.minplayers,
    item.maxplayers,
    poll_bestwith.result_value as poll_bestplayers,
    poll_recommendedwith.result_value as poll_recommendedplayers,
    itemmechanic.value as mechanics,
    itemfamily.value as families,
    itemcategory.value as categories,
    item.yearpublished,
    item.playingtime,
    item.minplaytime,
    item.maxplaytime,
    item.minage,
    item.usersrated, 
    item.average, 
    item.bayesaverage, 
    item.stddev, 
    item.median, 
    item.owned, 
    item.trading, 
    item.wanting, 
    item.wishing, 
    item.numcomments, 
    item.numweights 
from item
left outer join itemname on item.id == itemname.item_id 
left outer join itemmechanic on item.id = itemmechanic.item_id
left outer join itemfamily on item.id = itemfamily.item_id
left outer join itemcategory on item.id = itemcategory.item_id
left outer join poll_bestwith on item.id = poll_bestwith.item_id
left outer join poll_recommendedwith on item.id = poll_recommendedwith.item_id
where itemname.name_type == "primary";

--join item_link on link.id == item_link.link_id 
--join item on item_link.item_id == item.id         

-- select link.value from link where link.link_type = "boardgamemechanic"
