use std::path::PathBuf;

use rstest::rstest;

const EXPECTED_COMMENT: &str = "// expect: ";

#[rstest]
fn lox_files(#[files("res/**/*.lox")] path: PathBuf) {
    let string_path = path.to_str().expect("Expected non emtpy path");

    let lox_source =
        std::fs::read_to_string(&path).expect(&format!("Expected to find a test {string_path}"));

    // Perhaps using matches would be cleaner
    let expected_stdout: Vec<&str> = lox_source
        .split('\n')
        .filter(|line| line.contains(EXPECTED_COMMENT))
        .map(|line| &line[line.find(EXPECTED_COMMENT).unwrap() + EXPECTED_COMMENT.len()..])
        .collect();

    assert_eq!(expected_stdout.join("\n"), "Working");
}
