use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Items {
    #[serde(rename = "@termsofuse")]
    terms_of_use: String,
    pub item: Item,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "@type")]
    item_type: String,
    #[serde(rename = "@id")]
    pub id: u32,
    thumbnail: Option<String>,
    image: Option<String>,
    description: Option<String>,
    yearpublished: Option<ValueField>,
    minplayers: Option<ValueField>,
    maxplayers: Option<ValueField>,
    playingtime: Option<ValueField>,
    minplaytime: Option<ValueField>,
    maxplaytime: Option<ValueField>,
    minage: Option<ValueField>,
    #[serde(rename = "name")]
    pub names: Vec<Name>,
    #[serde(rename = "poll")]
    polls: Vec<Poll>,
    #[serde(rename = "link")]
    pub links: Vec<Link>,
}

impl Item {
    pub fn primary_name(&self) -> String {
        for n in &self.names {
            if n.name_type == "primary" {
                return n.value.clone();
            }
        }
        panic!("{self:#?}")
    }
    pub fn mechanics(&self) -> Vec<Mechanic> {
        self.links
            .iter()
            .filter(|l| l.link_type == "boardgamemechanic")
            .map(|l| Mechanic {
                name: l.value.clone(),
                id: l.id,
            })
            .collect()
    }
}

pub struct Mechanic {
    pub name: String,
    pub id: u32,
}

pub struct Category {
    name: String,
    id: u32,
}
pub struct Family {
    name: String,
    id: u32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Name {
    #[serde(rename = "@type")]
    name_type: String,
    #[serde(rename = "@sortindex")]
    sortindex: u32,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
struct ValueField {
    #[serde(rename = "@value")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
struct Poll {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@title")]
    title: String,
    #[serde(rename = "@totalvotes")]
    totalvotes: u32,
    results: Option<Vec<Results>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
struct Results {
    numplayers: Option<String>,
    result: Vec<Result>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
struct Result {
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@numvotes")]
    numvotes: u32,
    #[serde(rename = "level")]
    level: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Link {
    #[serde(rename = "@type")]
    link_type: String,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@value")]
    value: String,
    #[serde(default)]
    inbound: bool,
}
