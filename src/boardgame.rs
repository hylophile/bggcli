use serde::{Deserialize, Serialize};

// impl Item {
//     pub fn primary_name(&self) -> String {
//         for n in &self.names {
//             if n.name_type == "primary" {
//                 return n.value.clone();
//             }
//         }
//         panic!("{self:#?}")
//     }
//     pub fn mechanics(&self) -> Vec<Mechanic> {
//         self.link
//             .iter()
//             .filter(|l| l.link_type == "boardgamemechanic")
//             .map(|l| Mechanic {
//                 name: l.value.clone(),
//                 id: l.id,
//             })
//             .collect()
//     }
// }

// pub struct Mechanic {
//     pub name: String,
//     pub id: i64,
// }

// pub struct Category {
//     name: String,
//     id: i64,
// }
// pub struct Family {
//     name: String,
//     id: i64,
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseItems {
    #[serde(rename = "@termsofuse")]
    pub termsofuse: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone)]
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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Name {
    #[serde(rename = "@type")]
    pub name_type: String,
    #[serde(rename = "@sortindex")]
    pub sortindex: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Yearpublished {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Minplayers {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Maxplayers {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PollSummary {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub result: Vec<PollSummaryResult>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PollSummaryResult {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Playingtime {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Minplaytime {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Maxplaytime {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Minage {
    #[serde(rename = "@value")]
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Results {
    #[serde(rename = "@numplayers")]
    pub numplayers: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub result: Vec<ResultsResult>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResultsResult {
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@numvotes")]
    pub numvotes: String,
    #[serde(rename = "@level")]
    pub level: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
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
