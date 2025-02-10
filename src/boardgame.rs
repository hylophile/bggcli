use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseItems {
    #[serde(rename = "@termsofuse")]
    pub termsofuse: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    #[serde(rename = "@type")]
    pub item_type: String,
    #[serde(rename = "@id")]
    pub id: i64,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub thumbnail: String,
    pub image: String,
    pub name: Vec<Name>,
    pub description: String,
    pub yearpublished: Yearpublished,
    pub minplayers: Minplayers,
    pub maxplayers: Maxplayers,
    #[serde(rename = "poll-summary")]
    pub poll_summary: PollSummary,
    pub playingtime: Playingtime,
    pub minplaytime: Minplaytime,
    pub maxplaytime: Maxplaytime,
    pub minage: Minage,
    pub poll: Vec<Poll>,
    pub link: Vec<Link>,
    pub statistics: Statistics,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Name {
    #[serde(rename = "@type")]
    pub name_type: String,
    #[serde(rename = "@sortindex")]
    pub sortindex: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Yearpublished {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Minplayers {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Maxplayers {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PollSummary {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub result: Vec<PollSummaryResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PollSummaryResult {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Playingtime {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Minplaytime {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Maxplaytime {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Minage {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Poll {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@totalvotes")]
    pub totalvotes: i64,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub results: Vec<Results>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Results {
    #[serde(rename = "@numplayers")]
    pub numplayers: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub result: Vec<ResultsResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultsResult {
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@numvotes")]
    pub numvotes: i64,
    #[serde(rename = "@level")]
    pub level: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    #[serde(rename = "@type")]
    pub link_type: String,
    #[serde(rename = "@id")]
    pub id: i64,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@inbound")]
    pub inbound: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Statistics {
    #[serde(rename = "@page")]
    pub page: i64,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub ratings: Ratings,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ratings {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub usersrated: Usersrated,
    pub average: Average,
    pub bayesaverage: Bayesaverage,
    pub ranks: Ranks,
    pub stddev: Stddev,
    pub median: Median,
    pub owned: Owned,
    pub trading: Trading,
    pub wanting: Wanting,
    pub wishing: Wishing,
    pub numcomments: Numcomments,
    pub numweights: Numweights,
    pub averageweight: Averageweight,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Usersrated {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Average {
    #[serde(rename = "@value")]
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bayesaverage {
    #[serde(rename = "@value")]
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ranks {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub rank: Vec<Rank>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rank {
    #[serde(rename = "@type")]
    pub rank_type: String,
    #[serde(rename = "@id")]
    pub id: i64,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@friendlyname")]
    pub friendlyname: String,
    #[serde(rename = "@value")]
    pub value: String, // "Not Ranked" or i64
    #[serde(rename = "@bayesaverage")]
    pub bayesaverage: String, // "Not Ranked" or f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stddev {
    #[serde(rename = "@value")]
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Median {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Owned {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trading {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wanting {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wishing {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Numcomments {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Numweights {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Averageweight {
    #[serde(rename = "@value")]
    pub value: f64,
}
