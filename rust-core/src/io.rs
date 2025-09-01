use std::{
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

// DEFINITIONS -----------------------------------------------------------------

/// Collection of filesystem metadata objects
pub struct MetaCollection<'a> {
    meta_files: Vec<&'a MetaFile>,
}

/// Individual metadata object on a filesystem
pub struct MetaFile {
    path: PathBuf,
}

/// Exposes metadata required for processing on related objects
pub trait Metadata {
    fn path(&self) -> &impl AsRef<Path>;
}

// IMPLEMENTATIONS -------------------------------------------------------------

impl MetaFile {
    /// Load a [`MetaFile`] from the given path
    /// 
    /// Returns [`Ok`] if the file can be loaded, otherwise an [`Error`] 
    /// explaining why it cannot be loaded.
    pub fn load<S: AsRef<Path>>(path: S) -> Result<MetaFile> {
        let pathstr: &Path = path.as_ref();
        match std::fs::exists(pathstr) {
            Ok(true) => Ok(MetaFile {
                path: pathstr.to_owned(),
            }),
            Ok(false) => Err(Error::new(ErrorKind::NotFound, "Not Found")),
            Err(e) => Err(e),
        }
    }

    /// The filesystem path associated with this [`MetaFile`]
    pub fn path(&self) -> &impl AsRef<Path> {
        &self.path
    }
}

impl Metadata for MetaFile {
    /// The filesystem path associated with this [`Metadata`]
    fn path(&self) -> &impl AsRef<Path> {
        self.path()
    }
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const TEST_DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/");

    #[test]
    fn exists() {}

    #[test]
    fn load_meta_file() {
        let path: PathBuf = PathBuf::from(TEST_DATA).join("test_1k_00.fits");
        assert!(MetaFile::load(path).is_ok());
    }

    #[test]
    fn load_meta_dir() {
        todo!();
        // load dir path
        // create metadata collection from metas in dir
        // check metadata collection length to match number of meta files in dir
    }
}
