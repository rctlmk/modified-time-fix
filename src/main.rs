use clap::{crate_authors, crate_version, Arg, Command};
use modified_time_fix::{fix_modified_time, is_elevated, walk_and_fix_modified_time, Date};

fn main() -> std::io::Result<()> {
    let elevated = is_elevated();

    let cmd = Command::new("mtime-fix")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Changes the last modification date of a directory.")
        .after_help(
            "This utility reads the entries of the target directory, finds an entry with \
        the latest (or earliest) modification date, and then replaces target directory's \
        modification date with that date. If the target directory is empty, then no \
        action is taken.",
        )
        .arg(
            Arg::new("path")
                .help("Target directory")
                .required(true)
                .multiple_occurrences(false),
        )
        .arg(
            Arg::new("earliest")
                .display_order(0)
                .long("earliest")
                .short('e')
                .visible_alias("min")
                .help("Set the last modification date to the earliest date")
                .takes_value(false)
                .conflicts_with("latest")
                .required(false),
        )
        .arg(
            Arg::new("latest")
                .display_order(1)
                .long("latest")
                .short('l')
                .visible_alias("max")
                .help("Set the last modification date to the latest date (default)")
                .takes_value(false)
                .conflicts_with("earliest")
                .required(false)
                .multiple_occurrences(false),
        )
        .arg(
            Arg::new("unsafe")
                .display_order(2)
                .long("unsafe")
                .short('u')
                .help("Enable unsafe mode (required if the program was launched with elevated privileges)")
                .takes_value(false)
                .required(elevated)
                .multiple_occurrences(false),
        )
        .subcommand(
            Command::new("recursive")
                .short_flag('R')
                .display_order(0)
                .about("Traverse directories and change last modification date recursively")
                .long_about(
                    "Traverse directories and change their last modification date recursively. Any encountered \
                IO error will fail the whole operation.",
                ),
        );

    let matches = match cmd.try_get_matches() {
        Ok(m) => m,
        Err(err) => {
            if err.use_stderr() {
                let _ = err.print();

                eprintln!("\nPress [ENTER] / [RETURN] to continue...");
                use std::io::BufRead;
                let mut s = String::new();
                let i = std::io::stdin();
                i.lock().read_line(&mut s).unwrap();

                std::process::exit(1);
            } else {
                let _ = err.print();
                std::process::exit(0);
            }
        },
    };

    let mut target = std::path::PathBuf::from(matches.value_of("path").unwrap());
    if target.exists() {
        if target.is_relative() {
            target = std::env::current_dir()?.join(target);
        }
    } else {
        eprintln!("Path \"{}\" doesn't exist.", target.display());
        std::process::exit(1);
    }

    let target = target.canonicalize().expect("io error");

    let date = if matches.is_present("earliest") {
        Date::Earliest
    } else {
        Date::Latest
    };

    let result = if matches.subcommand_matches("recursive").is_some() {
        walk_and_fix_modified_time(target, date)
    } else {
        fix_modified_time(target, date)
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
