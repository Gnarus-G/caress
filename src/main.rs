use clap::Parser;
use std::{fs, io, path};

#[derive(Parser)]
#[clap(version, about, author)]
/// A tool to touch a file while creating any necessary directories automatically
struct Args {
    /// The file path to touch.
    path: path::PathBuf,
}

fn main() {
    let args = Args::parse();
    if let Err(err) = mk_file(args.path) {
        println!("{}", err);
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
