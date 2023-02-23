use std::{
    fs::read_dir,
    io::{Error, ErrorKind},
    path::Path,
};

use crate::lib::{terminal, Fallible, GlobPattern};

pub enum ListArguments {
    /// List all folders that match a specific pattern from a directory.
    ///
    /// format: `--glob=[[glob]]` e.g.
    /// ```bash
    /// #! lists all Rust files.
    /// coco ls [...target?] --glob=[*.rs]
    /// ```
    GlobPattern(GlobPattern),
    /// Also recursively open all children directories and list their contents.
    /// format: `--recursive` e.g.
    /// ```bash
    /// coco ls [...target?] --recursive
    /// ```
    Recursive,
    /// Set depth of file tree to open.
    /// format: `--depth=[number]` e.g.
    /// ```bash
    /// #! Lists all files 3 levels deep.
    /// coco ls [...target?] --depth=3
    /// ```
    Depth(usize),
}

/// List the files in a directory.
pub fn list_path(path: &Path, arguments: &Vec<ListArguments>) -> Fallible<()> {
    if !path.exists() {
        let message = format!(
            "Could not find the directory path \"{}\".",
            path.to_str().unwrap()
        );
        return Err(Error::new(ErrorKind::NotFound, message));
    }
    if !path.is_dir() {
        let message = format!("\"{}\" is not a directory.", path.to_str().unwrap());
        return Err(Error::new(ErrorKind::Other, message));
    }
    for entry in path.read_dir()? {
        println!("{}", entry?.path().to_str().unwrap());
    }
    Ok(())
}
