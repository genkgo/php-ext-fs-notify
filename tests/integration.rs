use phper_test::{cli::test_long_term_php_script_with_condition, utils::get_lib_path};
use std::{
    env,
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};
use std::io::{Write, Read, Seek, SeekFrom};
use tempfile::NamedTempFile;

#[test]
fn test_recommended_watcher() {
    let mut tmpfile = NamedTempFile::new().unwrap();
    write!(tmpfile, "").unwrap();

    test_long_term_php_script_with_condition(
        get_lib_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target"),
            "php_ext_fs_notify",
        ),
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("recommended_watcher.php"),
        |_| {
            sleep(Duration::from_secs(3));
            let mut tmpfile_watch = tmpfile.reopen().unwrap();
            write!(tmpfile_watch, "notice").unwrap();
        }
    );

    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("notice", buf);
}