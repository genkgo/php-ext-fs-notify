use phper_test::{cli::test_php_scripts, utils::get_lib_path};
use std::{
    env,
    path::{Path, PathBuf},
};

#[test]
fn test_recommended_watcher() {
    test_php_scripts(
        get_lib_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target"),
            "php_ext_fs_notify",
        ),
        &[&Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("recommended_watcher.php")],
    );
}