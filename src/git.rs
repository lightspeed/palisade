use anyhow::{anyhow, Result};
use git2::{Cred, RemoteCallbacks, Repository, Signature};

/// Push all tags in the repo to the upstream origin.
pub(crate) fn push_tag(repo: &Repository, token: &String, tag: &String) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_u, _username_from_url, allowed_types| {
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            let user = "git";
            git2::Cred::ssh_key_from_agent(user)
        } else {
            Cred::userpass_plaintext(&token, "")
        }
    });
    let mut remote = repo.find_remote("origin")?;
    let mut po = git2::PushOptions::new();
    po.remote_callbacks(callbacks);

    match remote.push(
        &[&format!("refs/tags/{}:refs/tags/{}", tag, tag)],
        Some(&mut po),
    ) {
        Ok(_) => Ok(()),
        Err(why) => Err(anyhow!("git push error: {:?}", why)),
    }
}

/// Tag the HEAD commit with a given version and description.
pub(crate) fn tag_version<T, U, V, W>(
    repo: &Repository,
    username: T,
    email: U,
    tag: V,
    desc: W,
) -> Result<()>
where
    T: Into<String>,
    U: Into<String>,
    V: Into<String>,
    W: Into<String>,
{
    let sig = &Signature::now(&username.into(), &email.into())?;
    let obj = repo.revparse_single("HEAD")?;
    repo.tag(&tag.into(), &obj, &sig, &desc.into(), false)?;

    Ok(())
}

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
        const USERNAME: &'static str = "Palisade";
        const EMAIL: &'static str = "p@lisa.de";

        let dir = tempdir()?;
        let repo = Repository::init(&dir)?;
        let mut fout = File::create(&dir.path().join("VERSION"))?;
        write!(fout, "{}", TAG)?;
        drop(fout);
        let mut index = repo.index()?;
        index.add_path(Path::new("VERSION"))?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;

        let sig = &Signature::now(USERNAME, EMAIL)?;
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "test commit please ignore",
            &tree,
            &[],
        )?;

        super::tag_version(&repo, USERNAME, EMAIL, TAG, format!("version {}", TAG))?;
        assert!(super::has_tag(&repo, &TAG.to_string())?);

        Ok(())
    }
}
