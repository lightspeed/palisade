use anyhow::Result;
use serde::{Deserialize, Serialize};
use reqwest::header;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRelease {
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
}

pub struct Client {
    cli: reqwest::Client,
}

// Name the user agent after this app
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")); 

impl Client {
    pub(crate) fn new(token: String) -> Result<Client> {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("token {}", &token))?);
        let cli = reqwest::Client::builder()
           .user_agent(APP_USER_AGENT)
           .default_headers(headers)
           .build()?;

        Ok(Client{
            cli: cli,
        })
    }

    fn create_release(&self, cr: CreateRelease) {}
}
