use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) mod cmd;
pub(crate) mod changelog;
pub(crate) mod git;
pub(crate) mod github;
pub(crate) mod version;

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = Cmd::from_args();

    match cmd {
        Cmd::Cut { common, changelog } => {
            cmd::cut::run(common, changelog).await
        }
    }
}
