use std::{
    fs::read_dir,
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
    slice::Iter,
};

// DEFINITIONS -----------------------------------------------------------------

/// Individual metadata object on a filesystem
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct MetaFile {
    path: PathBuf,
}

/// Collection of filesystem metadata objects
pub struct MetaCollection {
    meta_files: Box<[MetaFile]>,
}

/// Exposes metadata required for processing on related objects
pub trait Metadata {
    /// The filesystem path associated with this [`Metadata`]
    fn path(&self) -> &Path;

    /// The name of the metadata file (the final component of the path)
    fn filename(&self) -> Option<&str> {
        self.path().file_name()?.to_str()
    }

    /// The file extension of the metadata file, without the leading '.'
    fn file_extension(&self) -> Option<&str> {
        self.path().extension()?.to_str()
    }
}

// IMPLEMENTATIONS -------------------------------------------------------------

impl MetaFile {
    /// Load a [`MetaFile`] from the given path
    ///
    /// Returns [`Ok`] ([`MetaFile`]) if the file can be loaded, otherwise
    /// an [`Err`] ([`Error`]) explaining why it cannot be loaded.
    pub fn load(path: impl AsRef<Path>) -> Result<MetaFile> {
        let path = path.as_ref();
        if path.exists() {
            Ok(MetaFile {
                path: path.to_owned(),
            })
        } else {
            Err(Error::new(ErrorKind::NotFound, "Filepath not found"))
        }
    }

    /// The filesystem path associated with this [`MetaFile`]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Metadata for MetaFile {
    fn path(&self) -> &Path {
        self.path()
    }
}

impl MetaCollection {
    /// Load all [`MetaFile`] inside of a directory and add them to a
    /// new collection
    ///
    /// Returns [`Ok`] ([`MetaCollection`]) if the files can be loaded,
    /// otherwise an [Err] ([`Error`]) explaining the issue.
    pub fn load(path: impl AsRef<Path>) -> Result<MetaCollection> {
        let mut metafiles: Vec<MetaFile> = Vec::new();
        for entry in read_dir(&path)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            metafiles.push(MetaFile::load(&path)?);
        }
        metafiles.sort();
        Ok(MetaCollection {
            meta_files: metafiles.into_boxed_slice(),
        })
    }

    /// Iterator that allows iteration over metadata in the collection
    pub fn iter(&self) -> Iter<'_, MetaFile> {
        self.meta_files.iter()
    }

    /// Number of metadatas in the collection
    pub fn count(&self) -> usize {
        self.meta_files.len()
    }
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    const TEST_DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/");

    #[test]
    fn exists() {}

    #[test]
    fn load_meta_file() {
        let filename = "test_1k_00.fits";
        let path = PathBuf::from(TEST_DATA).join(filename);
        let metafile = MetaFile::load(path).unwrap();
        assert!(metafile.filename().unwrap() == "test_1k_00.fits");
        assert!(metafile.file_extension().unwrap() == "fits");
    }

    #[test]
    fn load_meta_dir() {
        // load dir path
        // create metadata collection from metas in dir
        // check metadata collection length to match number of meta files in dir
        let mut paths: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(TEST_DATA).unwrap() {
            let entry = entry.unwrap();
            if !entry.file_type().unwrap().is_file() {
                continue;
            }
            paths.push(entry.path());
        }
        paths.sort();
        let paths = paths;

        let collection = MetaCollection::load(TEST_DATA).unwrap();
        assert!(collection.count() == paths.len());
    }
}
