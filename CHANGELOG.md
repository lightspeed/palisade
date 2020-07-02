# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### FIXED

- Email addresses and usernames for git signatures are now configurable

## 0.2.0

### ADDED

- Palisade now manages its own releases
- CircleCI support

## 0.1.0

This is the first release of palisade!

### ADDED

- Initial implementation of the `cut` command
- Single-call GitHub API client for creating releases
- Support for operations on git repos using
  [git2](https://docs.rs/git2/0.13.6/git2/)
- Builds with [Nix](https://nixos.org/nix)
- Documentation and project layout
- TL;DR Rust document
- GitHub Action support
- Commentary across the project
- Support using SSH keys for authentication to GitHub
