use anyhow::Result;
use reqwest::header;
use serde::{Deserialize, Serialize};

/// The inputs to https://developer.github.com/v3/repos/releases/#create-a-release
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRelease {
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
}

/// Release is an individual release of a GitHub repo.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub id: i64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub author: Author,
    pub assets: Vec<Asset>,
}

/// Asset is a GitHub release asset.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub url: String,
    pub browser_download_url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub state: String,
    pub content_type: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub author: Author,
}

/// The author/creator of a release.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,
}

/// Client interacts with the GitHub API as described in https://developer.github.com/v3/
/// asynchronously.
pub struct Client {
    cli: reqwest::Client,
}

// Name the user agent after this app
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

impl Client {
    /// Construct a new Client with an API token.
    pub(crate) fn new(token: String) -> Result<Client> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("token {}", &token))?,
        );
        let cli = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()?;

        Ok(Client { cli: cli })
    }

    /// Creates a new GitHub release following the schema here:
    /// https://developer.github.com/v3/repos/releases/#create-a-release
    pub(crate) async fn create_release(
        &self,
        owner: String,
        repo: String,
        cr: CreateRelease,
    ) -> Result<Release> {
        let result: Release = self
            .cli
            .post(&format!(
                "https://api.github.com/repos/{}/{}/releases",
                owner, repo
            ))
            .json(&cr) // auto-magically json-encodes the CreateRelease argument from the caller into the request body
            .send()
            .await?
            .error_for_status()? // returns an error if the response code isn't 2xx
            .json() // decodes the response as json into a Release using serde
            .await?;

        Ok(result)
    }
}
