use std::{
    fs, io,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};

pub fn mkdir(path: &Path) -> io::Result<()> {
    let mut dirs = path
        .iter()
        .scan(PathBuf::new(), |path, s| {
            path.push(s);
            Some(path.clone())
        })
        .collect::<Vec<_>>();

    let mut need_to_create = vec![];

    while let Some(path) = dirs.last() {
        if let Err(err) = fs::create_dir(path) {
            if err.kind() == io::ErrorKind::NotFound {
                if let Some(s) = dirs.pop() {
                    need_to_create.push(s);
                    continue;
                }
            }
        }

        break;
    }

    for s in need_to_create.iter().rev() {
        fs::create_dir(s)?;
    }

    Ok(())
}

pub fn mkdir_alt(path: &mut PathBuf) -> io::Result<()> {
    match fs::create_dir(&path) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            if let Some(p) = path.parent() {
                mkdir_alt(&mut PathBuf::from(p))?;
                fs::create_dir(path)?;
            }
            Ok(())
        }
        e => e,
    }
}

pub fn mkdir_alt2(path: &Path) -> io::Result<()> {
    let mut p_buf = PathBuf::new();
    let mut check_dir = true;

    for s in path.iter() {
        p_buf.push(s);
        if !check_dir || (check_dir && !p_buf.is_dir()) {
            fs::create_dir(&p_buf)?;
            check_dir = false;
        }
    }

    Ok(())
}

pub mod test_utils {

    use super::*;

    #[derive(Debug)]
    pub struct TestDirs {
        namespace: &'static str,
        pub path: PathBuf,
    }

    impl TestDirs {
        pub fn new(namespace: &'static str, num_dirs: usize) -> Self {
            let path_components_base = vec!["testDirs".to_string(), namespace.to_string()];

            let path_components_sub_dirs: Vec<String> =
                (0..=num_dirs).map(|n| n.to_string()).collect();

            let path_str = [path_components_base, path_components_sub_dirs]
                .concat()
                .join(&MAIN_SEPARATOR.to_string());

            Self {
                namespace,
                path: PathBuf::from(path_str),
            }
        }

        pub fn remove(self) {
            fs::remove_dir_all(Path::new("testDirs").join(self.namespace)).unwrap();
        }
    }
}