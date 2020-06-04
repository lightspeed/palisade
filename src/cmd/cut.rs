use anyhow::{anyhow, Result};
use crate::*;
use std::path::PathBuf;

pub(crate) async fn run(common: Common, fname: PathBuf) -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let tag = version::read_version("VERSION".into())?;
    let desc = changelog::read(fname.clone(), tag.clone())?;

    if !git::has_tag(&repo, tag.clone())? {
        git::tag_version(&repo, tag.clone(), desc.clone())?;
        git::push_tags(&repo)?;
    } else /* the tag exists in the repo */ {
        return Ok(());
    }

    let gh = github::Client::new(common.token);

    Ok(())
}
