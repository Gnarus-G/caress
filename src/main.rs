use clap::Parser;
use std::{
    fs, io,
    path::{self, MAIN_SEPARATOR},
};

#[derive(Parser)]
#[command(version, about, author)]
/// A tool to touch a file, or create a directory, while creating any necessary directories in the middle automatically
struct Args {
    /// The path to touch.
    path: path::PathBuf,

    /// Echo the path given.
    #[clap(short, long)]
    echo: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = if path_is_dir(&args.path) {
        fs::create_dir_all(&args.path)
    } else {
        mk_file(&args.path)
    } {
        return eprintln!("{err}");
    }

    if args.echo {
        return print!("{}", args.path.to_str().unwrap_or_default());
    }
}

fn mk_file(path: &path::PathBuf) -> io::Result<()> {
    if let Err(err) = fs::File::create(&path) {
        if err.kind() == io::ErrorKind::NotFound {
            let mut temp = path.clone();
            temp.pop();
            fs::create_dir_all(temp)?;
            fs::File::create(path)?;
        } else {
            return Err(err);
        }
    }

    return Ok(());
}

#[inline]
fn path_is_dir(p: &path::PathBuf) -> bool {
    p.to_str()
        .expect("path should be a valid string")
        .ends_with(MAIN_SEPARATOR)
}
