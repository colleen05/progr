use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use crate::{utils::*, PROGR_PATH_PREFIX};

#[derive(Debug)]
pub struct Stage {
    pub name: String,
    pub items: HashSet<String>,
}

impl Stage {
    pub fn new(name: &str, items: HashSet<String>) -> Self {
        Self {
            name: name.into(),
            items,
        }
    }

    pub fn path_from_name(name: &str) -> PathBuf {
        [".", PROGR_PATH_PREFIX, "stages", name].iter().collect()
    }

    pub fn path(&self) -> PathBuf {
        Self::path_from_name(&self.name)
    }

    pub fn open(name: &str) -> io::Result<Stage> {
        let contents = fs::read_to_string(Self::path_from_name(name))?;

        let items: HashSet<String> = contents
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Stage::new(name, items))
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
            self.items
                // Escape newlines and spaces
                .iter()
                .map(|t| t.replace(' ', "-").replace('\n', "\\n"))
                .collect::<Vec<String>>()
                // Separate by newline
                .join("\n")
                .as_bytes(),
        )?;
        f.write_all("\n".as_bytes())?;

        Ok(())
    }
}

pub fn find_stages() -> Vec<String> {
    find_objects_by_typename("stages").unwrap_or_default()
}
