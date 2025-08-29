use std::io::*;

pub struct MetaCollection<'a> {
    meta_files: Vec<&'a MetaFile>,
}

pub struct MetaFile {
    filepath: String,
}

pub trait MetaData {
    fn path<'a>(&'a self) -> &'a str;
}

impl MetaData for MetaFile {
    fn path<'a>(&'a self) -> &'a str {
        &self.filepath
    }
}

pub fn load_meta_file(path: &str) -> Result<MetaFile> {
    match std::fs::exists(path) {
        Ok(true) => Ok(MetaFile {
            filepath: path.to_owned(),
        }),
        Ok(false) => Err(Error::new(ErrorKind::NotFound, "Not Found")),
        Err(e) => Err(e),
    }
}

/// UNIT TESTS -----------------------------------------------------------------

#[cfg(test)]
mod tests {

    const TEST_DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/");

    #[test]
    fn exists() {}

    fn load_meta_file() {
        let path: String = TEST_DATA.to_owned() + "test_1k_00.fits";
        if let Err(_) = super::load_meta_file(&path) { panic!() };
		todo!();
    }

    fn load_meta_dir() {
        todo!();
    }
}
