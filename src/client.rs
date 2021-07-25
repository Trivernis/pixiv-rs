use reqwest::Client;
use crate::error::{Result, Error};
use crate::responses::{Response, Illustration};
use serde::de::DeserializeOwned;
use serde::Serialize;

static BASE_URL: &str = "https://www.pixiv.net/ajax";
static LANG_PARAM: (&str, &str) = ("lang", "en");

#[derive(Clone, Debug)]
pub struct PixivClient {
    client: Client
}

impl PixivClient {
    pub fn new() -> Self {
        Self {client: Client::new()}
    }

    /// Returns data for the given illustration
    pub async fn illustration<S: AsRef<str>>(&self, id: S) -> Result<Illustration> {
        self.get(format!("illust/{}", id.as_ref()), &[("full", "1"), LANG_PARAM]).await
    }

    async fn get<T: DeserializeOwned, S: AsRef<str>, Q: Serialize + ?Sized>(&self, path: S, query: &Q) -> Result<T> {
        let response = self.client.get(format!("{}/{}", BASE_URL, path.as_ref())).query(query).send().await?;

        let json_response = response.json::<Response<T>>().await?;
        if json_response.error {
            Err(Error::Pixiv(json_response.message))
        } else {
            Ok(json_response.body.get().unwrap())
        }
    }
}