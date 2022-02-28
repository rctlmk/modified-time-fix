# modified-time-fix

Changes the last modification date of a directory.

```
ARGS:
    <path>    Target directory

OPTIONS:
    -e, --earliest    Set the last modification date to the earliest date [aliases: min]
    -l, --latest      Set the last modification date to the latest date (default) [aliases: max]
    -u, --unsafe      Enable unsafe mode (required if the program was launched with elevated privileges)
    -h, --help        Print help information
    -V, --version     Print version information

SUBCOMMANDS:
    recursive -R    Traverse directories and change last modification date recursively
    help            Print this message or the help of the given subcommand(s)

This utility reads the entries of the target directory, finds an entry with the latest (or earliest)
modification date, and then replaces target directory's modification date with that date. If the
target directory is empty, then no action is taken.
```

## Compiling from source

Make sure you have these installed:

* `git`
* [`cargo` and `rustc`](https://www.rust-lang.org/tools/install)

Clone the repository: 

```
git clone https://github.com/rctlmk/modified-time-fix.git
cd modified-time-fix
```

And build with `cargo`:

```
cargo build --release
```

## License

Distributed under the terms of the MIT license, see [LICENSE](LICENSE) for details.