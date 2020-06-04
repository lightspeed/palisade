use crate::*;
use anyhow::Result;
use std::path::PathBuf;

pub(crate) async fn run(common: Common, fname: PathBuf) -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let tag = version::read_version("VERSION".into())?;
    let desc = changelog::read(fname, &tag)?;

    if !git::has_tag(&repo, &tag)? {
        git::tag_version(&repo, &tag, &desc)?;
        git::push_tags(&repo)?;
    } else
    /* the tag exists in the repo */
    {
        return Ok(());
    }

    let gh = github::Client::new(common.token)?;

    gh.create_release(common.owner, common.name, github::CreateRelease{
        tag_name: tag.clone(),
        target_commitish: "HEAD".into(),
        name: format!("Version {}", tag),
        body: desc,
        draft: false,
        prerelease: false,
    }).await?;

    Ok(())
}
