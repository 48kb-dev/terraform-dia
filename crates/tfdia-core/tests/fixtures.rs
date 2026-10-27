//! Snapshot tests: every directory in `fixtures/` is parsed and its JSON model snapshotted.
//! Review changes with `cargo insta review`.

use std::fs;
use std::path::Path;

use tfdia_core::{SourceFile, parse};

#[test]
#[ignore = "enable once parser::parse is implemented"]
fn fixtures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let mut dirs: Vec<_> = fs::read_dir(&root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();

    for dir in dirs {
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let model = parse(&read_tf_files(&dir)).unwrap();
        insta::assert_json_snapshot!(name, model);
    }
}

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
