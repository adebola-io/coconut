use std::{
    env::{args, Args},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use super::{
    filesystem::{
        delete::{delete_path, DeleteArguments},
        list::{list_path, ListArguments},
    },
    terminal,
    utils::Fallible,
    Command,
};

pub struct Config {
    main_command: Command,
    current_directory: PathBuf,
}

/// Build a necessary configuration for the process.
pub fn build_config(current_directory: PathBuf) -> Fallible<Config> {
    let mut args = args();
    let main_command;
    if let Some(_) = args.next() {
        if let Some(command_string) = args.next() {
            match command_string.as_str() {
                "del" | "delete" => {
                    main_command = Command::delete_command(args)?;
                    Ok(Config {
                        main_command,
                        current_directory,
                    })
                }
                "ls" | "list" => {
                    main_command = Command::list_command(&current_directory, args)?;
                    Ok(Config {
                        main_command,
                        current_directory,
                    })
                }
                "cls" | "clear" => {
                    main_command = Command::Clear;
                    Ok(Config {
                        main_command,
                        current_directory,
                    })
                }
                _ => {
                    let message = format!("Cannot recognise the command \"{}\".", command_string);
                    Err(Error::new(ErrorKind::Other, message))
                }
            }
        } else {
            main_command = Command::Help;
            Ok(Config {
                main_command,
                current_directory,
            })
        }
    } else {
        Err(Error::new(
            ErrorKind::InvalidData,
            "The path to the binary directory could not be determined.",
        ))
    }
    // Arg 1.
}

impl Config {
    pub fn run(&self) -> Fallible<()> {
        match &self.main_command {
            Command::Delete { targets, arguments } => {
                for target in targets {
                    let path = target.as_path();
                    delete_path(&path)?;
                    terminal::success(format!("Deleted {}.", path.to_str().unwrap()));
                }
                Ok(())
            }
            Command::List { targets, arguments } => {
                for target in targets {
                    list_path(&target, arguments)?;
                }
                Ok(())
            }
            Command::Mkdir { target, name } => todo!(),
            Command::Clear => todo!(),
            Command::Help => todo!(),
            Command::Run {} => todo!(),
        }
    }
}
