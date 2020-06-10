use std::path::PathBuf;
use structopt::StructOpt;

mod cmd;
mod changelog;
mod git;
mod github;
mod version;

/// Common arguments across subcommands.
#[derive(StructOpt, Debug)]
pub(crate) struct Common {
    /// GitHub token to authenticate with
    #[structopt(long, short, env="GITHUB_TOKEN")]
    pub token: String,
    /// Repo owner
    #[structopt(long, short="O", env="REPO_OWNER")]
    pub owner: String,
    /// Repo name
    #[structopt(long, short="R", env="REPO_NAME")]
    pub name: String
}

#[derive(StructOpt, Debug)]
pub(crate) struct GitHubAction {
    // these are set by GitHub Actions:
    // https://help.github.com/en/actions/configuring-and-managing-workflows/using-environment-variables
    /// GitHub repository in owner/repo format
    #[structopt(env="GITHUB_REPOSITORY")]
    pub repo: String,
    /// GitHub token to authenticate with
    #[structopt(env="GITHUB_TOKEN")]
    pub token: String,
    /// GitHub ref
    #[structopt(env="GITHUB_REF")]
    pub refname: String,

    // these are set by the end user of this action
    /// Changelog filename relative to repo root
    #[structopt(env="CHANGELOG_FILENAME", default_value="./CHANGELOG.md")]
    changelog_fname: PathBuf,
}

// Conversion function for turning a GitHubAction into a Common
// This consumes the GitHubAction in many cases.
impl From<GitHubAction> for Common {
    fn from(gha: GitHubAction) -> Self {
        let owner_repo: Vec<&str> = gha.repo.split('/').collect();

        Common{
            token: gha.token,
            owner: owner_repo[0].to_string(),
            name: owner_repo[1].to_string(),
        }
    }
}

/// The possible subcommands for this tool. [structopt](https://docs.rs/structopt/0.3.14/structopt/)
/// is used to have these commands and arguments get parsed based on this
/// information.
#[derive(StructOpt, Debug)]
#[structopt(about = "A simple release management tool")]
pub(crate) enum Cmd {
    /// Creates a new release for a git repo
    Cut {
        #[structopt(flatten)]
        common: Common,
        /// Changelog location
        #[structopt(long, short, default_value="./CHANGELOG.md")]
        changelog: PathBuf,
    },

    /// Runs releases as triggered by GitHub Actions
    GitHubAction {
        #[structopt(flatten)]
        gha: GitHubAction,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = Cmd::from_args();

    match cmd {
        Cmd::Cut { common, changelog } => {
            cmd::cut::run(common, changelog).await
        }
        Cmd::GitHubAction { gha } => {
            cmd::github_action::run(gha).await
        }
    }
}
