use std::{
    env::Args,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use super::{
    filesystem::{delete::DeleteArguments, list::ListArguments},
    terminal, Fallible,
};

pub enum Command {
    /// Deletes a path recursively.
    /// ```bash
    /// coco delete [...targets] [...args]
    /// ```
    Delete {
        targets: Vec<PathBuf>,
        arguments: Vec<DeleteArguments>,
    },
    /// Lists all the files and folders in a directory.  
    /// ```bash
    /// coco ls (target?) [...args]
    /// ```
    /// OR
    /// ```bash
    /// coco list (target?) [...args]
    /// ```
    List {
        /// Defaults to current directory.
        targets: Vec<PathBuf>,
        arguments: Vec<ListArguments>,
    },
    /// Make a new directory.  
    /// ```bash
    /// coco mkdir (target?) [...args]
    /// ```
    Mkdir {
        /// Defaults to current directory.
        target: PathBuf,
        name: String,
    },
    /// Clear the terminal or console.
    /// ```bash
    /// coco clear
    /// ```
    /// OR
    /// ```bash
    /// coco cls
    /// ```
    Clear,
    Help,
    /// Run an external command.
    Run {},
}

impl Command {
    /// Create a delete command.
    pub fn delete_command(args: Args) -> Fallible<Self> {
        let mut targets = vec![];
        let mut warned = false;
        let mut arguments = vec![];
        for arg in args {
            if arg.starts_with("--") {
                match arg.as_str() {
                    _ => terminal::warn(format!("Invalid argument \"{}\".", arg)),
                }
            } else {
                targets.push(std::path::PathBuf::from(arg));
            }
        }
        if targets.len() == 0 {
            Err(Error::new(
                ErrorKind::NotFound,
                "Delete target not specified.",
            ))
        } else {
            Ok(Self::Delete { targets, arguments })
        }
    }
    /// Create a list command.
    pub fn list_command(current_directory: &PathBuf, args: Args) -> Fallible<Self> {
        let mut targets = vec![];
        let mut arguments = vec![];
        for arg in args {
            if arg.starts_with("--") {
                match arg.as_str() {
                    _ => terminal::warn(format!("Invalid argument \"{}\".", arg)),
                }
            } else {
                targets.push(PathBuf::from(arg))
            }
        }
        if targets.len() == 0 {
            targets.push(current_directory.clone())
        }
        Ok(Self::List { targets, arguments })
    }
}
