use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use crate::PROGR_PATH_PREFIX;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub tags: HashSet<String>,
    pub description: String,
}

impl Item {
    pub fn new(name: &str, tags: HashSet<String>, description: &str) -> Self {
        Self {
            name: name.into(),
            tags,
            description: description.into(),
        }
    }

    pub fn path_from_name(name: &str) -> PathBuf {
        [".", PROGR_PATH_PREFIX, "items", name].iter().collect()
    }

    pub fn path(&self) -> PathBuf {
        Self::path_from_name(&self.name)
    }

    pub fn open(name: &str) -> io::Result<Item> {
        let contents = fs::read_to_string(Self::path_from_name(name))?;
        let mut lines = contents.lines();

        let tags: HashSet<String> = lines
            .next()
            .unwrap_or_default()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let description: String = lines.collect();

        Ok(Item::new(name, tags, &description))
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

        f.write_all(
            self.tags
                // Escape newlines and spaces
                .iter()
                .map(|t| t.replace(' ', "-").replace('\n', "\\n"))
                .collect::<Vec<String>>()
                // Separate by space
                .join(" ")
                .as_bytes(),
        )?;
        f.write_all("\n".as_bytes())?;

        f.write_all(self.description.as_bytes())?;
        f.write_all("\n".as_bytes())?;

        Ok(())
    }
}
