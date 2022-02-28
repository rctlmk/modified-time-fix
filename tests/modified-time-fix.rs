use std::fs::File;
use std::path::Path;
use std::{fs, io};

use filetime::{set_file_handle_times, FileTime};
use lazy_static::lazy_static;
use modified_time_fix::{fix_modified_time, walk_and_fix_modified_time, Date};
use tempfile::{tempdir, tempdir_in, TempDir};

lazy_static! {
    /// Wednesday, January 1, 1969 00:00:00
    static ref SIXTY_NINE: FileTime = FileTime::from_unix_time(-31546800, 0);

    /// Sunday, January 1, 1984 00:00:00
    static ref EIGHTY_FOUR: FileTime = FileTime::from_unix_time(441752400, 0);
}

/// Creates temp files in `dir`.
fn temp_files_in(dir: impl AsRef<Path>) -> io::Result<(TempDir, File, File)> {
    let temp_dir = tempdir_in(dir)?;

    // for some reason these files are immediately deleted on my Arch WSL
    // let a = tempfile_in(&temp_dir)?;
    // let b = tempfile_in(&temp_dir)?;

    let a = File::create(temp_dir.path().join("a"))?;
    let b = File::create(temp_dir.path().join("b"))?;

    Ok((temp_dir, a, b))
}

/// Creates temp files in the temp directory.
fn temp_files() -> io::Result<(TempDir, File, File)> {
    temp_files_in(std::env::temp_dir())
}

/// Extracts last modification date from `path`'s metadata.
fn filetime(path: impl AsRef<Path>) -> FileTime {
    FileTime::from_last_modification_time(&fs::metadata(path).expect("unable to read metadata"))
}

/// Sets new last modification dates for temp files, calls [`fix_modified_time`], and compares
/// new date with expected.
fn set_and_check(date: Date, expected: FileTime) {
    let (temp_dir, a, b) = temp_files().expect("unable to create test files");

    set_file_handle_times(&a, None, Some(*EIGHTY_FOUR))
        .and(set_file_handle_times(&b, None, Some(*SIXTY_NINE)))
        .and(fix_modified_time(&temp_dir, date))
        .expect("unable to set file time");

    let actual = filetime(&temp_dir);

    assert_eq!(expected, actual);
}

#[test]
fn set_earliest_date() {
    set_and_check(Date::Earliest, *SIXTY_NINE);
}

#[test]
fn set_latest_date() {
    set_and_check(Date::Latest, *EIGHTY_FOUR);
}

#[test]
fn empty_directory_remains_unchanged() {
    let temp_dir = tempdir().expect("unable to create test files");

    let expected = filetime(&temp_dir);

    fix_modified_time(&temp_dir, Date::Earliest).expect("unable to set file time");

    let actual = filetime(&temp_dir);

    assert_eq!(expected, actual);
}

#[test]
fn set_date_recursively() {
    let root = tempdir().expect("unable to create test files");
    let (_sub_x, a, _b) = temp_files_in(&root).expect("unable to create test files");
    let sub_y = tempdir_in(&root).expect("unable to create test files");
    let (_sub_z, c, _d) = temp_files_in(&sub_y).expect("unable to create test files");

    set_file_handle_times(&a, None, Some(*EIGHTY_FOUR))
        .and(set_file_handle_times(&c, None, Some(*SIXTY_NINE)))
        .and(walk_and_fix_modified_time(&root, Date::Earliest))
        .expect("unable to set file time");
}
