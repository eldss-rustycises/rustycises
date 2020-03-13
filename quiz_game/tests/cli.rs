use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::process::Command; // Run programs // Used for writing assertions

#[test]
fn file_doesnt_exist() {
    let mut cmd = Command::cargo_bin("quiz_game").expect("Failed to build binary");
    cmd.arg("-c").arg("file/doesnt/exist.csv");
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("No such file or directory"));
}
