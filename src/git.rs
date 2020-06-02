use anyhow::Result;
use git2::{Repository, Signature};

pub(crate) fn push_tags(repo: &Repository) -> Result<()> {
    let mut remote = repo.find_remote("origin")?;
    remote.connect(git2::Direction::Push)?;
    remote.push(&["refs/tags/*:refs/tags/*"], None)?;
    Ok(())
}

pub(crate) fn tag_version(repo: &Repository, tag: String, desc: String) -> Result<()> {
    let sig = &Signature::now("Gitea Release Tool", "gitea-release@tulpa.dev")?;
    let obj = repo.revparse_single("HEAD")?;
    repo.tag(&tag, &obj, &sig, &desc, false)?;

    Ok(())
}

pub(crate) fn has_tag(repo: &Repository, tag: String) -> Result<bool> {
    let tags = repo.tag_names(Some(&tag))?;

    for tag_obj in tags.iter() {
        if tag_obj.is_none() {
            continue;
        }

        let tag_name = tag_obj.unwrap();
        if tag == tag_name.to_string() {
            return Ok(true);
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use git2::*;
    use std::{fs::File, io::Write, path::Path};
    use tempfile::tempdir;

    #[test]
    fn has_tag() -> Result<()> {
        const TAG: &'static str = "0.1.0";
        let dir = tempdir()?;
        let repo = Repository::init(&dir)?;
        let mut fout = File::create(&dir.path().join("VERSION"))?;
        write!(fout, "{}", TAG)?;
        drop(fout);
        let mut index = repo.index()?;
        index.add_path(Path::new("VERSION"))?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;

        let sig = &Signature::now("Gitea Release Tool", "gitea-release@tulpa.dev")?;
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "test commit please ignore",
            &tree,
            &[],
        )?;

        super::tag_version(&repo, TAG.into(), format!("version {}", TAG))?;
        assert!(super::has_tag(&repo, TAG.into())?);

        Ok(())
    }
}
