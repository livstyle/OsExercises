use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn cicvverify() {
    Command::cargo_bin("cicv")
        .unwrap()
        .args(&["--nocapture", "cicvverify"]) 
        // .current_dir("exercises")
        .assert()
        .success();
}
