use ::palisade::cmd::{self, Cmd};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = Cmd::from_args();

    match cmd {
        Cmd::Circle { ccie } => cmd::circleci::run(ccie).await,
        Cmd::Cut { common, changelog } => cmd::cut::run(common, changelog).await,
        Cmd::GithubAction { gha } => cmd::github_action::run(gha).await,
    }
}
