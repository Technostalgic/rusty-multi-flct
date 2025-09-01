use std::{
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

// DEFINITIONS -----------------------------------------------------------------

pub struct MetaCollection<'a> {
    meta_files: Vec<&'a MetaFile>,
}

pub struct MetaFile {
    path: PathBuf,
}

pub trait Metadata {
    fn path(&self) -> &impl AsRef<Path>;
}

// IMPLEMENTATIONS -------------------------------------------------------------

impl Metadata for MetaFile {
    fn path(&self) -> &impl AsRef<Path> {
        &self.path
    }
}

pub fn load_meta_file<S: AsRef<Path>>(path: S) -> Result<MetaFile> {
    let pathstr: &Path = path.as_ref();
    match std::fs::exists(pathstr) {
        Ok(true) => Ok(MetaFile {
            path: pathstr.to_owned(),
        }),
        Ok(false) => Err(Error::new(ErrorKind::NotFound, "Not Found")),
        Err(e) => Err(e),
    }
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    const TEST_DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/");

    #[test]
    fn exists() {}

    #[test]
    fn load_meta_file() {
        let path: PathBuf = PathBuf::from("");
        assert!(super::load_meta_file(path).is_ok())
    }

    #[test]
    fn load_meta_dir() {
        todo!();
        // load dir path
        // create metadata collection from metas in dir
        // check metadata collection length to match number of meta files in dir
    }
}
