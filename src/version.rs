use anyhow::Result;
use std::{fs, path::PathBuf};

pub(crate) fn read_version(fname: PathBuf) -> Result<String> {
    let version = fs::read_to_string(fname)?;
    Ok(version.trim().into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_version() {
        let version = super::read_version("./testdata/VERSION".into()).unwrap();
        assert_eq!(version, "0.1.0");
    }
}
