use anyhow::Result;
use git2::Repository;

/// Returns Ok(true) if the given repository has the given tag.
pub(crate) fn has_tag(repo: &Repository, tag: &String) -> Result<bool> {
    let tags = repo.tag_names(Some(&tag))?;

    for tag_obj in tags.iter() {
        if tag_obj.is_none() {
            continue;
        }

        let tag_name = tag_obj.unwrap();
        if *tag == tag_name.to_string() {
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

    /// Creates a git repo and tests the has_tag and tag_version functions.
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

        let sig = &Signature::now("Palisade", "p@lisa.de")?;
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "test commit please ignore",
            &tree,
            &[],
        )?;

        let desc = format!("version {}", TAG);
        repo.tag(&tag, &obj, &sig, &desc, false)?;
        assert!(super::has_tag(&repo, &TAG.to_string())?);

        Ok(())
    }
}
