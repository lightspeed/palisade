use anyhow::Result;
use github::*;
use git2::*;
use std::{env, fs::File, io::Write, path::Path};
use tempfile::tempdir;

const GH_TOKEN_NAME: &'static str = "GITHUB_TOKEN";

#[test]
fn dependencies() {
    assert!(env::var(GH_TOKEN_NAME).is_ok());
}

fn to_clone_url(u: String) -> Result<String> {
    let mut u = url::Url::parse(&u)?;
    u.set_username(&env::var(GH_TOKEN_NAME)?).unwrap();
    Ok(u.to_string())
}

#[tokio::test]
async fn cut() -> Result<()> {
    const TAG: &'static str = "0.1.0";
    let token = env::var(GH_TOKEN_NAME)?;

    let name = elfs::next();
    let cli = Client::new(token.clone().into())?;
    let gh_repo: github::Repo = cli.create_repo(RepoCreate{
        name: name,
        description: "integration test for palisade, please ignore".into(),
        homepage: "https://github.com/lightspeed/palisade".into(),
        private: true,
        has_issues: true,
        has_projects: true,
        has_wiki: true,
    }).await?;

    // set up auth for github repo
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_u, _username_from_url, allowed_types| {
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            let user = "git";
            git2::Cred::ssh_key_from_agent(user)
        } else {
            Cred::userpass_plaintext(&token, "")
        }
    });
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // clone git repo to tempdir
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    let dir = tempdir()?;
    let repo = builder.clone(&to_clone_url(gh_repo.html_url)?, dir.path())?;

    // create VERSION/CHANGELOG files
    let mut fout = File::create(&dir.path().join("VERSION"))?;
    write!(fout, "{}", TAG)?;
    drop(fout);

    let mut fout = File::create(&dir.path().join("CHANGELOG.md"))?;
    fout.write(include_bytes!("../testdata/basic.md"))?;
    drop(fout);

    // stage the files
    let mut index = repo.index()?;
    index.add_path(Path::new("CHANGELOG.md"))?;
    index.add_path(Path::new("VERSION"))?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;

    // commit the files
    let sig = &Signature::now("Palisade", "p@lisa.de")?;
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "test commit please ignore",
        &tree,
        &[],
    )?;

    // push the files
    let mut remote = repo.find_remote("origin")?;
    let mut po = git2::PushOptions::new();

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_u, _username_from_url, allowed_types| {
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            let user = "git";
            git2::Cred::ssh_key_from_agent(user)
        } else {
            Cred::userpass_plaintext(&token, "")
        }
    });

    po.remote_callbacks(callbacks);
    remote.push(
        &["refs/heads/master:refs/heads/master"], Some(&mut po),
    )?;

    // okay, now we can test the cut function
    env::set_current_dir(dir.path())?;
    let _ = palisade::cmd::cut::run(palisade::Common{
        token: token.clone(),
        owner: gh_repo.owner.login.clone(),
        name: gh_repo.name.clone(),
    }, "CHANGELOG.md".into()).await?;

    // check that the release actually exists
    let release = cli.newest_release(gh_repo.owner.login.clone(), gh_repo.name.clone()).await?;

    assert_eq!(
        release.body,
        "Hi there this is a test\\!\n### ADDED\n  - something\n"
    );

    // cleanup
    cli.delete_repo(gh_repo.owner.login.clone(), gh_repo.name.clone()).await?;

    Ok(())
}
