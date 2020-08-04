use crate::{git, changelog, version, cmd::*};
use github::*;
use anyhow::Result;
use std::path::PathBuf;

/// Cuts a new release with GitHub details and a changelog filename.
pub async fn run(common: Common, fname: PathBuf) -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let tag = version::read_version("VERSION")?;
    let vtag = format!("v{}", tag);
    let desc = changelog::read(fname, &tag)?;

    if git::has_tag(&repo, &vtag)? || git::has_tag(&repo, &tag)? {
        /* the tag exists in the repo */
        println!("{} already exists as a git tag, exiting", vtag);
        return Ok(());
    }

    let gh = Client::new(common.token)?;

    let release = gh.create_release(common.owner, common.name, CreateRelease{
        tag_name: vtag.clone(),
        target_commitish: "master".into(), // XXX(Christine): this may need to become an argument somehow.
        name: format!("Version {}", tag),
        body: desc,
        draft: false,
        prerelease: false,
    }).await?;

    println!("created release for {}: {}", vtag, release.html_url);

    Ok(())
}
