# Using Palisade

Migrating an existing project to use palisade for release management is an easy
process. You only need to do a few steps which will be covered in detail below.
At a high level you need to do the following:

- Set up the CHANGELOG file
- Set up the VERSION file
- Set up Palisade to run in CI

Afterwards, you can follow the release process listed below in the "Release
management" section.

## Setup

Palisade works by having two files in your repository that it uses to track
release information. These are the list of changes made to the software (the
changelog) and the current version of the software (the version file). In order
to set up your project to use palisade, you need to create these files.

### Set up the CHANGELOG file

The changelog file is based on [keepachangelog's
format](https://keepachangelog.com/en/1.0.0/). You can use this template to
create a new changelog compatible with palisade:

```markdown
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
```

Save this as `CHANGELOG.md` in the root of the repository.

### Set up the VERSION file

The version file contains one line, the current semantic version string of the
software. This information is normally managed outside the git repository, but
palisade puts this inside the git repository so that software version updates
can be reviewed alongside software releases.

If the current version of your software is `0.1.0`, add the following to your
version file in `./VERSION`:

```
0.1.0
```

If the current version of your software is already set, use this version number
instead of `0.1.0`.

### Set up Palisade to run in CI

Both of the configurations below will configure palisade to automatically run
after tests pass on your default branch. You will need a GitHub personal access
token with the `repo` permission associated to a user that has the "Maintain"
permission for the repository you want to do releases for.

#### GitHub Actions

Palisade can be run on GitHub Actions without any special setup. Be sure to have
this step run only on your default branch. You will need to put a personal
access token with the `repo` permission in a secret named `GH_TOKEN`.

Something like this configuration should suffice:

```yaml
jobs:
  test:
    steps:
      - uses: actions/checkout@v2
      - name: Run tests
        run: cargo test
  release:
    needs: test
    if: github.ref == 'refs/heads/master'
    steps:
      - name: Releases via Palisade
        uses: docker://lightspeedretail/palisade
        env:
          GITHUB_TOKEN: ${{ secrets.GH_TOKEN }}
        with:
          args: github-action
```

If your default branch is not named `master`, you will need to adjust the `if`
value in the Palisade step to the name of your default branch. Here are a few
examples:

- `if: github.ref == 'refs/heads/main'`
- `if: github.ref == 'refs/heads/trunk'`
- `if: github.ref == 'refs/heads/develop'`
- `if: github.ref == 'refs/heads/edge'`

#### CircleCI

Running this on CircleCI requires using a physical machine to run the release
job. The release job will look something like this:

```yaml
jobs:
  release:
    machine: true
    steps:
      - checkout
      - add_ssh_keys:
          fingerprints:
          - # Add your SSH key fingerprint here
      - run:
          name: "Publish Release on GitHub"
          command: |
            docker run --rm -itv $(pwd):/code --workdir /code \
              -e GITHUB_TOKEN \
              -e CIRCLE_PROJECT_USERNAME \
              -e CIRCLE_PROJECT_REPONAME \
              -e CIRCLE_BRANCH \
              --volume $SSH_AUTH_SOCK:/ssh-agent \
              --env SSH_AUTH_SOCK=/ssh-agent \
              lightspeedretail/palisade \
              palisade circle
          
workflows:
  version: 2
  tests:
    jobs:
      - build
      - release:
          requires:
            - build
          filters:
            branches:
              only:
                - master
```

If your project uses a non-master branch as the default branch, replace `master`
above with the name if the project's default branch.

## Release management

Palisade is a tool designed to automate release management. Therefore
interacting with it requires you to manage changelog and version files with the
normal git flow (branches and pull requests). For this example, we will be
updating the version of this program from version `0.1.0` to version `0.2.0`.

To release a new version of a program, first update the version file to the
desired version. Open the `VERSION` file with your favorite text editor and
replace:

```
0.1.0
```

With:

```
0.2.0
```

Then update the changelog file with the changes in that version. For example if
version `0.2.0` added the ability to interface with clients using GraphQL, you
could add this to your `CHANGELOG.md`:

```
## 0.2.0

### ADDED

- Exposed GraphQL API for customers and internal integrators

### FIXED

- Solved WAT-2392 which previously prevented users from being able to
  refrobnicate already frobnicated strings when using the secret management API.
```

When palisade runs, it will load the contents of the VERSION file and compare it
to the list of git tags in the repo. If that version tag is not found, then it
will create a new GitHub release with the changelog entry for the new version.

This would create a release for tag `v0.2.0` with the following notes:

```
### ADDED

- Exposed GraphQL API for customers and internal integrators

### FIXED

- Solved WAT-2392 which previously prevented users from being able to
  refrobnicate already frobnicated strings when using the secret management API.
```

You can then have any triggers that run on a new tag being created (such as
packages being built or version bump pull requests being made). This is used
in Lightspeed in order to automate version management for a few of our internal
tooling projects.
