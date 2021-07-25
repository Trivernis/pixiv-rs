use serde::Deserialize;
use std::fmt::Debug;
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct Response<T> {
    pub error: bool,
    pub message: String,
    pub body: ResponseBody<T>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseBody<T> {
    WithValue(T),
    NoValue(Vec<()>)
}

impl<T> ResponseBody<T> {
    pub fn get(self) -> Option<T> {
        match self {
            ResponseBody::WithValue(b) => {Some(b)}
            ResponseBody::NoValue(_) => {None}
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Illustration {
    pub id: String,
    pub title: String,
    pub description: String,
    pub illust_type: u32,
    pub create_date: String,
    pub upload_date: String,
    #[serde(alias="xRestrict")]
    pub age_restrict: u32,
    pub urls: IllustrationUrl,
    pub tags: IllustrationTags,
    pub alt: String,
    pub user_id: String,
    pub user_name: String,
    pub user_account: String,
    pub width: u32,
    pub height: u32,
    pub page_count: u32,
    pub bookmark_count: u64,
    pub like_count: u64,
    pub comment_count: u64,
    pub response_count: u64,
    pub view_count: u64,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IllustrationUrl {
    pub mini: String,
    pub thumb: String,
    pub small: String,
    pub regular: String,
    pub original: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct IllustrationTags {
    pub author_id: String,
    pub tags: Vec<Tag>
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Tag {
    pub tag: String,
    pub user_id: Option<String>,
    pub romaji: Option<String>,
    #[serde(default)]
    pub translation: HashMap<String, String>,
}