use anyhow::Result;
use std::{fs, path::PathBuf};

/// Parses the VERSION file into a string without trailing newlines.
pub(crate) fn read_version<T>(fname: T) -> Result<String> where T: Into<PathBuf> {
    let version = fs::read_to_string(fname.into())?;
    Ok(version.trim().into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_version() {
        let version = super::read_version("./testdata/VERSION").unwrap();
        assert_eq!(version, "0.1.0");
    }
}
