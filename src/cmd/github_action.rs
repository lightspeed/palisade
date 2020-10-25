use crate::*;
use anyhow::Result;
use github::Client;

/// Handle the auto-release functionality when run as a GitHub Action.
/// See https://help.github.com/en/actions/creating-actions/creating-a-docker-container-action
/// for more information.
pub async fn run(gha: GitHubAction) -> Result<()> {
    let cli = Client::new(gha.token.clone())?;

    let owner_repo: Vec<&str> = gha.repo.split('/').collect();
    let repo = cli
        .get_repo(owner_repo[0].to_string(), owner_repo[1].to_string())
        .await?;
    let refname = format!("refs/heads/{}", repo.default_branch);

    if gha.refname != refname {
        println!(
            "doesn't need to run on non-default branches, refname is {} and wanted {}",
            refname, gha.refname
        );
        return Ok(());
    }

    let changelog_fname = gha.changelog_fname.clone();
    let common: Common = gha.into();

    cmd::cut::run(common, changelog_fname).await
}
