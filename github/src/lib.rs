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

/// Inputs to create a GitHub repository.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoCreate {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub private: bool,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
}

/// A minimal view of the repo on GitHub.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    pub full_name: String,
    pub owner: Author,
    pub description: String,
    pub homepage: String,
    pub private: bool,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub html_url: String,
}

/// Client interacts with the GitHub API as described in https://developer.github.com/v3/
/// asynchronously.
pub struct Client {
    cli: reqwest::Client,
    base_url: String,
}

// Name the user agent after this app
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

impl Client {
    /// Construct a new Client with an API token.
    pub fn new(token: String) -> Result<Client> {
        Client::with_url(token, "https://api.github.com/".into())
    }

    pub fn with_url(token: String, base_url: String) -> Result<Client> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("token {}", &token))?,
        );
        let cli = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()?;

        Ok(Client {
            cli: cli,
            base_url: base_url,
        })
    }

    /// Creates a new GitHub repo following the schema here:
    /// https://developer.github.com/v3/repos/#create-a-repository-for-the-authenticated-user
    pub async fn create_repo(&self, rc: RepoCreate) -> Result<Repo> {
        let result: Repo = self
            .cli
            .post(&format!("{}user/repos", self.base_url))
            .json(&rc)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(result)
    }

    /// Deletes a GitHub repo following the schema here:
    /// https://developer.github.com/v3/repos/#delete-a-repository
    pub async fn delete_repo(&self, owner: String, repo: String) -> Result<()> {
        self.cli
            .delete(&format!("{}repos/{}/{}", self.base_url, owner, repo))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(())
    }

    /// Creates a new GitHub release following the schema here:
    /// https://developer.github.com/v3/repos/releases/#create-a-release
    pub async fn create_release(
        &self,
        owner: String,
        repo: String,
        cr: CreateRelease,
    ) -> Result<Release> {
        let result: Release = self
            .cli
            .post(&format!(
                "{}repos/{}/{}/releases",
                self.base_url, owner, repo
            ))
            .json(&cr) // auto-magically json-encodes the CreateRelease argument from the caller into the request body
            .send()
            .await?
            .error_for_status()? // returns an error if the response code isn't 2xx
            .json() // decodes the response as json into a Release using serde
            .await?;

        Ok(result)
    }

    /// Gets the most recent release for a GitHub repo following the schema here:
    /// https://developer.github.com/v3/repos/releases/#get-the-latest-release
    pub async fn newest_release(&self, owner: String, repo: String) -> Result<Release> {
        let result: Release = self
            .cli
            .get(&format!(
                "{}repos/{}/{}/releases/latest",
                self.base_url, owner, repo
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(result)
    }
}

#[cfg(test)] // conditionally compiles if and only if tests are being run
mod tests {
    use super::*;
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[tokio::test]
    async fn create_repo() {
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/create_repo.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("POST", "/user/repos"))
                .respond_with(json_encoded(data)),
        );

        let cli = Client::with_url("testswag420".into(), format!("{}", server.url("/"))).unwrap();
        cli.create_repo(RepoCreate {
            name: "swagalicious".into(),
            description: "a test repo".into(),
            homepage: "https://yolo.swag".into(),
            private: true,
            has_issues: false, // this software is perfect
            has_projects: false,
            has_wiki: false,
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn delete_repo() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("DELETE", "/repos/yolo/swag"))
                .respond_with(status_code(204)),
        );
        let cli = Client::with_url("testswag420".into(), format!("{}", server.url("/"))).unwrap();
        cli.delete_repo("yolo".into(), "swag".into()).await.unwrap();
    }

    #[tokio::test]
    async fn create_release() {
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/create_release.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("POST", "/repos/yolo/swag/releases"))
                .respond_with(json_encoded(data)),
        );

        let cli = Client::with_url("testswag420".into(), format!("{}", server.url("/"))).unwrap();
        cli.create_release(
            "yolo".into(),
            "swag".into(),
            CreateRelease {
                tag_name: "v4.2.0".into(),
                target_commitish: "HEAD".into(),
                name: "my awesome release".into(),
                body: "yeah this is a test".into(),
                draft: false,
                prerelease: false,
            },
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn newest_release() {
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/create_release.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/repos/yolo/swag/releases/latest",
            ))
            .respond_with(json_encoded(data)),
        );

        let cli = Client::with_url("testswag420".into(), format!("{}", server.url("/"))).unwrap();
        cli.newest_release("yolo".into(), "swag".into())
            .await
            .unwrap();
    }
}
