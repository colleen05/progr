use std::{collections::HashSet, io};

use clap::Args;
use progr::prelude::*;

#[derive(Args)]
pub struct InitArgs {
    /// Create .gitkeep files in object directories.
    #[arg(short = 'k', long)]
    pub create_gitkeeps: bool,
}

pub fn init(args: &InitArgs) -> io::Result<()> {
    let ws = Workspace {
        stages: vec![
            Stage::new("concept", HashSet::new()),
            Stage::new("todo", HashSet::new()),
            Stage::new("progress", HashSet::new()),
            Stage::new("done", HashSet::new()),
        ],
        stages_order: vec![
            "concept".into(),
            "todo".into(),
            "progress".into(),
            "done".into(),
        ],
        default_stage: "todo".into(),
        ..Default::default()
    };

    ws.write_all()?;

    if args.create_gitkeeps {
        ws.create_gitkeeps()?;
    } else {
        ws.create_missing_directories()?;
    }

    Ok(())
}
