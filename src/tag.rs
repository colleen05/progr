use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use crate::{utils::*, PROGR_PATH_PREFIX};

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub description: String,
}

impl Tag {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }

    pub fn path_from_name(name: &str) -> PathBuf {
        [".", PROGR_PATH_PREFIX, "tags", name].iter().collect()
    }

    pub fn path(&self) -> PathBuf {
        Self::path_from_name(&self.name)
    }

    pub fn open(name: &str) -> io::Result<Tag> {
        let contents = fs::read_to_string(Self::path_from_name(name))?;
        let description = contents.strip_suffix('\n').unwrap_or(&contents);

        Ok(Tag::new(name, description))
    }

    pub fn write(&self) -> io::Result<()> {
        // Prepare parent directory.
        if let Some(parent_dir) = self.path().parent() {
            fs::create_dir_all(parent_dir)?;
        } else {
            panic!("Expected parent directory.");
        }

        // Create the file.
        let mut f = File::create(self.path())?;

        f.write_all(self.description.as_bytes())?;
        f.write_all("\n".as_bytes())?;

        Ok(())
    }
}

pub fn find_tags() -> Vec<String> {
    find_objects_by_typename("tags").unwrap_or_default()
}
