use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(test)]
mod tests {
    use caress::test_utils::TestDirs;

    use super::*;

    fn assert_dir_path_create(path: &Path) {
        assert!(path.exists());

        path.iter()
            .scan(PathBuf::new(), |path, s| {
                path.push(s);
                Some(path.clone())
            })
            .for_each(|s| {
                println!("{s:?}");
                assert!(s.exists())
            });
    }

    #[test]
    fn create_dirs_with_stdlib() {
        let td = TestDirs::new("stdlib-method", 10);

        fs::create_dir_all(&td.path).unwrap();

        assert_dir_path_create(&td.path);

        td.remove();
    }

    mod custom_fns {
        use caress::mkdir;
        use caress::mkdir_alt;
        use caress::mkdir_alt2;

        use super::*;

        #[test]
        fn create_dirs_with_mkdir() {
            let td = TestDirs::new("custom-mkdir-method", 10);
            let path = &td.path;

            mkdir(&mut path.clone()).unwrap();

            assert_dir_path_create(path);

            td.remove();
        }

        #[test]
        fn create_dirs_with_mkdir_and_some_dirs_exist() {
            let td = TestDirs::new("tfc1", 10);
            let path = &td.path;

            fs::create_dir_all(path.parent().unwrap().parent().unwrap()).unwrap();

            mkdir(&mut path.clone()).unwrap();

            assert_dir_path_create(path);

            td.remove();
        }

        #[test]
        fn create_dirs_with_mkdir_alt() {
            let td = TestDirs::new("tt", 10);
            let path = &td.path;

            mkdir_alt(path).unwrap();

            assert_dir_path_create(&path);

            td.remove();
        }

        #[test]
        fn create_dirs_with_mkdir_alt2() {
            let td = TestDirs::new("tt2", 10);
            let path = &td.path;

            mkdir_alt2(path).unwrap();

            assert_dir_path_create(&path);

            td.remove();
        }
    }
}
