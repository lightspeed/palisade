use crate::*;
use anyhow::Result;

/// Handle the auto-release functionality when run as a GitHub Action.
/// See https://help.github.com/en/actions/creating-actions/creating-a-docker-container-action
/// for more information.
pub async fn run(gha: GitHubAction) -> Result<()> {
    if gha.refname != "refs/heads/master" {
        println!("doesn't need to run on non-master branches");
        return Ok(());
    }

    let changelog_fname = gha.changelog_fname.clone();
    let common: Common = gha.into();

    cmd::cut::run(common, changelog_fname).await
}
