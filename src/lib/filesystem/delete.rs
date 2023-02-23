use std::{fs, io, path::Path};

use crate::lib::GlobPattern;

pub enum DeleteArguments {
    /// Delete all folders that match a specific pattern from a directory.
    ///
    /// format: `--glob=[glob]` e.g.
    /// ```bash
    /// #! deletes all Typescript files.
    /// coco del (target) --glob=[*.ts]
    /// ```
    GlobPattern(GlobPattern),
}

/// Delete a file or a folder recursively.
pub fn delete_path(path: &Path) -> Result<(), io::Error> {
    if !path.exists() {
        return Err(io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            format!(
                "The system cannot find \"{}\". It may have been moved or already deleted.",
                path.display()
            ),
        ));
    }
    if path.is_dir() {
        // Delete paths recursively.
        for sub_path in fs::read_dir(path)? {
            delete_path(&sub_path?.path())?
        }
        fs::remove_dir(path)?
    } else {
        fs::remove_file(path)?
    }
    Ok(())
}
