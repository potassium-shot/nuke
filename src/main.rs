use std::{io::Write, path::Path};

use clap::Parser;
use rand::Rng;

mod args;

fn main() -> Result<(), std::io::Error> {
    let args = args::Args::parse();
    nuke(args.file, args.tick_size, args.no_delete)
}

fn nuke(path: impl AsRef<Path>, tick_size: u64, no_delete: bool) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    if std::fs::metadata(path)?.is_dir() {
        nuke_directory(path, tick_size, no_delete)
    } else {
        nuke_file(path, tick_size, no_delete)
    }
}

fn nuke_directory(
    path: impl AsRef<Path>,
    tick_size: u64,
    no_delete: bool,
) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    for element in std::fs::read_dir(path)? {
        let element = element?;

        if element.file_type()?.is_dir() {
            nuke_directory(element.path(), tick_size, no_delete)?;
        } else {
            nuke_file(element.path(), tick_size, no_delete)?;
        }
    }

    if !no_delete {
        std::fs::remove_dir(path)?;
    }

    Ok(())
}

fn nuke_file(
    path: impl AsRef<Path>,
    tick_size: u64,
    no_delete: bool,
) -> Result<(), std::io::Error> {
    let path = path.as_ref();
    let mut file = std::fs::File::options()
        .write(true)
        .create(false)
        .open(path)?;
    let mut remaining = file.metadata()?.len();

    while remaining > 0 {
        let to_nuke = u64::min(tick_size, remaining);
        let buf: Vec<u8> = rand::rngs::OsRng::default()
            .sample_iter::<u8, _>(rand::distributions::Standard)
            .take(to_nuke as usize)
            .collect();
        remaining -= to_nuke;
        file.write_all(&buf)?;
    }

    if !no_delete {
        std::fs::remove_file(path)?;
    }

    Ok(())
}
