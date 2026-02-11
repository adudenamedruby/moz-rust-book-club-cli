use anyhow::Result;
use assert_cmd::{Command, cargo};
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;

const TEST_DIR: &str = "tests/expected/";

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::new(cargo::cargo_bin!("echor"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() {
    let mut cmd = Command::new(cargo::cargo_bin!("echor"));
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() -> Result<()> {
    run_file_test(&["Hello there"], "hello1.txt")
}

#[test]
fn hello2() -> Result<()> {
    run_file_test(&["Hello", "there"], "hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    run_file_test(&["Hello  there", "-n"], "hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<()> {
    run_file_test(&["-n", "Hello", "there"], "hello2.n.txt")
}

fn run_file_test(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(format!("{}{}", TEST_DIR, expected_file))?;
    let output = Command::new(cargo::cargo_bin!("echor"))
        .args(args)
        .output()
        .expect("fail");

    let stout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stout, expected);

    Ok(())
}
