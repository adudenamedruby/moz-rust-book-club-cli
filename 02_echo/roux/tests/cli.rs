use assert_cmd::{Command, cargo};
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    let mut cmd = Command::new(cargo::cargo_bin!("echor"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Command::new(cargo::cargo_bin!("echor"));
    cmd.arg("hello").assert().success();
}
