//! Snapshot tests: every directory in `fixtures/` is parsed and its JSON model snapshotted.
//! Review changes with `cargo insta review`.

use std::fs;
use std::path::{Path, PathBuf};

use tfdia_core::{SourceFile, parse};

#[test]
fn fixtures_are_valid_hcl() {
    for dir in fixture_dirs() {
        for file in read_tf_files(&dir) {
            if let Err(e) = hcl::parse(&file.contents) {
                panic!("{}/{}: {e}", dir.display(), file.path);
            }
        }
    }
}

#[test]
#[ignore = "enable once parser::parse is implemented"]
fn fixtures() {
    for dir in fixture_dirs() {
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let model = parse(&read_tf_files(&dir)).unwrap();
        insta::assert_json_snapshot!(name, model);
    }
}

fn fixture_dirs() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let mut dirs: Vec<_> = fs::read_dir(&root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    dirs
}

/// Reads `*.tf` files directly in `dir` (not recursive, like the CLI).
fn read_tf_files(dir: &Path) -> Vec<SourceFile> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "tf"))
        .map(|p| SourceFile {
            path: p.file_name().unwrap().to_string_lossy().into_owned(),
            contents: fs::read_to_string(&p).unwrap(),
        })
        .collect();
    files.sort_by(|a, b| a.path.cmp(&b.path));
    files
}
