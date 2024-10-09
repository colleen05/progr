use std::{fs, io, path::PathBuf};

use crate::{prelude::*, PROGR_PATH_PREFIX};

#[derive(Debug, Default)]
pub struct Workspace {
    pub items: Vec<Item>,
    pub stages: Vec<Stage>,
    pub tags: Vec<Tag>,
    pub default_stage: String,
    pub notes: String,
    pub stages_order: Vec<String>,
}

impl Workspace {
    pub fn open() -> io::Result<Workspace> {
        let items = find_items()
            .iter()
            .map(|name| Item::open(name))
            .collect::<io::Result<Vec<Item>>>()?;

        let stages = find_stages()
            .iter()
            .map(|name| Stage::open(name))
            .collect::<io::Result<Vec<Stage>>>()?;

        let tags = find_tags()
            .iter()
            .map(|name| Tag::open(name))
            .collect::<io::Result<Vec<Tag>>>()?;

        let default_stage = fs::read_to_string(".progress/default_stage").unwrap_or_default();
        let notes = fs::read_to_string(".progress/notes").unwrap_or_default();

        let stages_order: Vec<String> = fs::read_to_string(".progress/stages_order")
            .unwrap_or_default()
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Ok(Self {
            items,
            stages,
            tags,
            default_stage,
            notes,
            stages_order,
        })
    }

    pub fn write_items(&self) -> io::Result<()> {
        for i in &self.items {
            i.write()?;
        }

        Ok(())
    }

    pub fn write_stages(&self) -> io::Result<()> {
        for s in &self.stages {
            s.write()?;
        }

        Ok(())
    }

    pub fn write_tags(&self) -> io::Result<()> {
        for t in &self.tags {
            t.write()?;
        }

        Ok(())
    }

    pub fn write_default_stage(&self) -> io::Result<()> {
        let path: PathBuf = [".", PROGR_PATH_PREFIX, "default_stage"].iter().collect();
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, &self.default_stage)
    }

    pub fn write_notes(&self) -> io::Result<()> {
        let path: PathBuf = [".", PROGR_PATH_PREFIX, "notes"].iter().collect();
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, &self.notes)
    }

    pub fn write_stages_order(&self) -> io::Result<()> {
        let path: PathBuf = [".", PROGR_PATH_PREFIX, "stages_order"].iter().collect();
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, &self.notes)
    }

    pub fn write_all(&self) -> io::Result<()> {
        self.write_items()?;
        self.write_stages()?;
        self.write_tags()?;
        self.write_default_stage()?;
        self.write_notes()?;
        self.write_stages_order()?;
        Ok(())
    }
}
