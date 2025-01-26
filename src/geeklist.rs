use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Geeklist {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@termsofuse")]
    terms_of_use: String,
    postdate: String,
    postdate_timestamp: u64,
    editdate: String,
    editdate_timestamp: u64,
    thumbs: u32,
    numitems: u32,
    username: String,
    title: String,
    description: String,
    item: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Item {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@objecttype")]
    object_type: String,
    #[serde(rename = "@subtype")]
    subtype: String,
    #[serde(rename = "@objectid")]
    object_id: u32,
    #[serde(rename = "@objectname")]
    object_name: String,
    #[serde(rename = "@username")]
    username: String,
    #[serde(rename = "@postdate")]
    postdate: String,
    #[serde(rename = "@editdate")]
    editdate: String,
    #[serde(rename = "@thumbs")]
    thumbs: u32,
    #[serde(rename = "@imageid")]
    image_id: String,
    body: String,
}
