use crate::*;
use anyhow::Result;

/// Handle the auto-release functionality when run under CircleCI.
/// See TODO(Christine): link to CircleCI docs here
/// for more information.
pub(crate) async fn run(ccie: CircleCIEnv) -> Result<()> {
    match ccie.branch.as_str() {
        "master" => {
            let changelog_fname = ccie.changelog_fname.clone();
            let common: Common = ccie.into();

            cmd::cut::run(common, changelog_fname).await
        }
        _ => {
            println!("don't need to run on {}", ccie.branch);
            Ok(())
        }
    }
}
