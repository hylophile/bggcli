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
    minage INTEGER NOT NULL
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


create view if not exists testing 
as 
select * from item 
inner join link 
on link.item_id == item.id 
order by item.id;

        
