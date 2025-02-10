use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Items {
    #[serde(rename = "@totalitems")]
    pub totalitems: String,
    #[serde(rename = "@termsofuse")]
    pub termsofuse: String,
    #[serde(rename = "@pubdate")]
    pub pubdate: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "@objecttype")]
    pub objecttype: String,
    #[serde(rename = "@objectid")]
    pub objectid: u32,
    #[serde(rename = "@subtype")]
    pub subtype: String,
    #[serde(rename = "@collid")]
    pub collid: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub name: Name,
    pub image: String,
    pub thumbnail: String,
    pub status: Status,
    pub numplays: String,
    pub yearpublished: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    #[serde(rename = "@sortindex")]
    pub sortindex: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "@own")]
    pub own: String,
    #[serde(rename = "@prevowned")]
    pub prevowned: String,
    #[serde(rename = "@fortrade")]
    pub fortrade: String,
    #[serde(rename = "@want")]
    pub want: String,
    #[serde(rename = "@wanttoplay")]
    pub wanttoplay: String,
    #[serde(rename = "@wanttobuy")]
    pub wanttobuy: String,
    #[serde(rename = "@wishlist")]
    pub wishlist: String,
    #[serde(rename = "@preordered")]
    pub preordered: String,
    #[serde(rename = "@lastmodified")]
    pub lastmodified: String,
}
