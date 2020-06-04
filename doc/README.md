# palisade

A simple release automation tool for GitHub repos

Palisade (IPA: /pæl əˈseɪd/) is a tool that reads from CHANGELOG and VERSION files
then uses them to cut releases of software. This tool is intended to be run by
CI tools on every commit to master.

## Raison d'Être

Existing automated release scripts for our GitHub repos have been flaky and have
not been able to populate the release notes from the changelog. Palisade
aims to automate the process of cutting releases down to pull requesting two
files in the repo: A [changelog](https://keepachangelog.com/en/1.0.0/) and a
version file. Once these files are updated on the master branch, then CI will use
this tool to bump the version if it needs to.

The changelog file will need to list version information in a second level
header. This will be used as the delimiters to scrape out the matching version
information. For example, with a changelog that looks like this:

```markdown
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0

### FIXED

- Refrobnicate the spurious rilkefs

## 0.0.1

First release, proof of concept.
```

When a release is created for version 0.1.0, this tool will make the description
of the release about as follows:

```
### FIXED

- Refrobnicate the spurious rilkefs
```

This allows the changelog file to be the ultimate source of truth for release
notes with this tool. Changes to the release notes can then be reviewed
alongside the source code of your project.

The `VERSION` file plays into this as well. The `VERSION` file MUST be a single
line containing a [semantic version][semver] string. This allows the `VERSION`
file to be the ultimate source of truth for software version data with this
tool. It also allows the software version to be reviewed alongside the source
code of your project, exposing something normally hidden in git to your team.

[semver]: https://semver.org/spec/v2.0.0.html

## Release Process

When this tool is run with the `cut` subcommand, the following actions take place:

- The `VERSION` file is read and loaded as the desired tag for the repo
- The `CHANGELOG.md` file (or the changelog path specified as a flag) is read
  and the changes for the `VERSION` are cherry-picked out of the file
- The git repo is checked to see if that tag already exists
  - If the tag exists, the tool exits and does nothing
- If the tag does not exist, it is created (with the changelog fragment as the
  body of the tag) and pushed to GitHub using locally stored credentials
- A GitHub release is created using the changelog fragment and the release name
  is generated from the `VERSION` string
