use std::{fs, io, path::PathBuf};

use crate::PROGR_PATH_PREFIX;

pub fn find_objects_by_typename(typename: &str) -> io::Result<Vec<String>> {
    let paths = fs::read_dir(
        [".", PROGR_PATH_PREFIX, typename]
            .iter()
            .collect::<PathBuf>(),
    )?;

    Ok(paths
        .filter_map(Result::ok)
        // Filter out non-files.
        .filter(|e| e.path().metadata().map(|m| m.is_file()).unwrap_or(false))
        // Get filenames without extension or leading path.
        .filter_map(|e| {
            e.path()
                .file_stem()
                .map(|s| s.to_str().map(|s| s.to_string()))
        })
        // Convert to Vec of String (object names).
        .flatten()
        .collect::<Vec<String>>())
}
