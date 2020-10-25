use crate::{cmd, *};
use anyhow::Result;
use github::Client;

/// Handle the auto-release functionality when run under CircleCI.
/// See TODO(Christine): link to CircleCI docs here
/// for more information.
pub async fn run(ccie: CircleCIEnv) -> Result<()> {
    let cli = Client::new(ccie.token.clone())?;
    let repo = cli.get_repo(ccie.owner.clone(), ccie.repo.clone()).await?;
    let default_branch = repo.default_branch;

    if ccie.branch == default_branch {
        let changelog_fname = ccie.changelog_fname.clone();
        let common: Common = ccie.into();

        cmd::cut::run(common, changelog_fname).await
    } else {
        println!("don't need to run on {} (wanted: {})", ccie.branch, default_branch);
        Ok(())
    }
}
