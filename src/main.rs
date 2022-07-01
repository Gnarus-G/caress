use clap::Parser;
use std::{fs, io, path};

#[derive(Parser)]
#[clap(version, about, author)]
/// A tool to touch a file, or create a directory, while creating any necessary directories in the middle automatically
struct Args {
    /// The path to touch.
    path: path::PathBuf,

    /// Create a directory.
    #[clap(short, long)]
    dir: bool,
}

fn main() {
    let args = Args::parse();

    let result = {
        if args.dir {
            fs::create_dir_all(&args.path)
        } else {
            mk_file(args.path)
        }
    };

    if let Err(err) = result {
        println!("{err}")
    }
}

fn mk_file(path: path::PathBuf) -> io::Result<()> {
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
