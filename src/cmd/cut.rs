use crate::*;
use anyhow::Result;
use std::path::PathBuf;

pub(crate) async fn run(common: Common, fname: PathBuf) -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let tag = version::read_version("VERSION".into())?;
    let vtag = format!("v{}", tag);
    let desc = changelog::read(fname, &tag)?;

    if !git::has_tag(&repo, &vtag)? {
        git::tag_version(&repo, &vtag, &desc)?;
        git::push_tag(&repo, &common.token, &vtag)?;
    } else
    /* the tag exists in the repo */
    {
        return Ok(());
    }

    let gh = github::Client::new(common.token)?;

    let release = gh.create_release(common.owner, common.name, github::CreateRelease{
        tag_name: tag.clone(),
        target_commitish: "master".into(),
        name: format!("Version {}", tag),
        body: desc,
        draft: false,
        prerelease: false,
    }).await?;

    println!("created release for {}: {}", vtag, release.html_url);

    Ok(())
}
