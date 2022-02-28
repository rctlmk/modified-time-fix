use std::path::{Path, PathBuf};
use std::{fs, io};

use filetime::{set_file_mtime, FileTime};

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod util;

#[derive(Debug, Clone, Copy)]
/// Date settings.
///
/// This type specifies date settings: `Earliest` variant represents the
/// earliest date, and `Latest` represents the latest date.
pub enum Date {
    /// The earliest date
    Earliest,
    /// The latest date
    Latest,
}

/// Checks if the current process has elevated privileges.
pub fn is_elevated() -> bool {
    util::is_elevated()
}

/// Updates last modification time of a directory.
///
/// Reads the entries in `dir`, then finds an entry with the latest
/// (or earliest, specified by the `date` parameter) modification date and
/// then replaces target directory's modification date with that date.
/// If the target directory is empty, then no action is taken.
///
/// # Errors
///
/// This function will return an error if:
///
/// * the provided `path` doesn't exist;
/// * the `path` points at a non-directory file;
/// * the process lacks permissions to perform necessary operations.
pub fn fix_modified_time(path: impl AsRef<Path>, date: Date) -> io::Result<()> {
    fs::read_dir(&path).and_then(|rd| {
        let iter = rd
            .filter_map(|rde| rde.ok())
            .filter_map(|de| de.metadata().ok())
            .map(|m| FileTime::from_last_modification_time(&m));

        let time = match date {
            Date::Earliest => iter.min(),
            Date::Latest => iter.max(),
        };

        time.map_or(Ok(()), |mtime| set_file_mtime(&path, mtime))
    })
}

/// Walks a file tree and updates last modification time of every directory.
/// This function will return after the first encountered error.
/// 
/// # Errors
/// 
/// See [`fix_modified_time`] for more details.
pub fn walk_and_fix_modified_time(dir_path: impl AsRef<Path>, date: Date) -> io::Result<()> {
    get_subdirs(dir_path)?
        .into_iter()
        .map(|p| fix_modified_time(p, date))
        .collect::<io::Result<_>>()
        .map(|_: Vec<()>| ())
}

/// Gets all subdirs (no depth limit) of `root`.
/// 
/// Traverses file tree using depth-first search algorithm (post order).
fn get_subdirs(root: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let mut stack = vec![root.as_ref().to_path_buf()];

    let mut output = vec![];

    while let Some(curr) = stack.pop() {
        let files = fs::read_dir(&curr)?;

        output.push(curr);

        let subdirs = files.filter_map(|rde| rde.ok()).map(|de| de.path()).filter(|p| p.is_dir());

        stack.extend(subdirs);
    }

    output.reverse();

    Ok(output)
}
