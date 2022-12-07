use clap::Parser;
use std::{
    fs, io,
    path::{self, MAIN_SEPARATOR},
};

#[derive(Parser)]
#[command(version, about, author)]
/**
A tool to touch a file, or create a directory, while creating any necessary directories in the middle automatically.
To create a directory, end the path with a '/'.
*/
struct Args {
    /// The paths to touch.
    paths: Vec<path::PathBuf>,

    /// Echo the paths given.
    #[clap(short, long)]
    echo: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err)
    }
}

fn run() -> std::io::Result<()> {
    let args = Args::parse();

    for path in args.paths {
        if path_is_dir(&path) {
            fs::create_dir_all(&path)?
        } else {
            touch(&path)?
        };

        if args.echo {
            println!("{}", path.to_str().unwrap_or_default());
        }
    }
    Ok(())
}

fn touch(path: &path::PathBuf) -> io::Result<()> {
    match fs::File::create(&path) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            let mut temp = path.clone();
            temp.pop();
            fs::create_dir_all(temp).and_then(|_| fs::File::create(path))?;
            Ok(())
        }
        Err(err) => Err(err),
        Ok(..) => Ok(()),
    }
}

#[inline]
fn path_is_dir(p: &path::PathBuf) -> bool {
    p.to_str()
        .expect("path should be a valid string")
        .ends_with(MAIN_SEPARATOR)
}
