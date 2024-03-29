use caress::mkdir;
use clap::Parser;
use std::{
    fs, io,
    path::{self, PathBuf, MAIN_SEPARATOR},
};

#[derive(Parser)]
#[command(version, about, author)]
/**
A tool to touch a file, or create a directory, while creating any necessary directories in the middle automatically.
To create a directory, end the path with a '/'.
*/
struct Args {
    /// The paths to touch.
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    for path in args.paths {
        let _ = if path_is_dir(&path) {
            mkdir(&mut path.clone())
                .map_err(|err| eprintln!("cannot create directory {path:?}: {err}"))
        } else {
            touch(&path).map_err(|err| eprintln!("cannot create file {path:?}: {err}"))
        };
    }
}

fn touch(path: &path::PathBuf) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }

    return match fs::File::create(&path) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            if let Some(dir) = path.parent() {
                mkdir(&mut dir.to_path_buf()).and_then(|_| fs::File::create(path))?;
            }
            Ok(())
        }
        Err(err) => Err(err),
        Ok(..) => Ok(()),
    };
}

#[inline]
fn path_is_dir(p: &path::PathBuf) -> bool {
    p.to_str()
        .expect("path should be a valid string")
        .ends_with(MAIN_SEPARATOR)
}
