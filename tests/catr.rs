use rand::{distributions::Alphanumeric, Rng};
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

const PRG: &str = "catr";
// const EMPTY: &str = "tests/data/empty.txt";
const FOX: &str = "tests/data/fox.txt";
const SPIDERS: &str = "tests/data/spiders.txt";
const BUSTLE: &str = "tests/data/bustle.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args_error() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// #[test]
// fn empty() -> TestResult {
//     run(&[EMPTY], EMPTY)
// }

// #[test]
// fn empty_stdin() -> TestResult {
//     run_stdin(EMPTY, &["-"], EMPTY)
// }

#[test]
fn fox() -> TestResult {
    run(&[FOX], FOX)
}

#[test]
fn fox_stdin() -> TestResult {
    run_stdin(FOX, &["-"], FOX)
}
#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], SPIDERS)
}

#[test]
fn spiders_stdin() -> TestResult {
    run_stdin(SPIDERS, &["-"], SPIDERS)
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], BUSTLE)
}

#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-"], BUSTLE)
}

// ---

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut expected = fs::read_to_string(expected_file)?;
    // catr will have an additional carriage return.
    expected = expected + "\n";
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let mut expected = fs::read_to_string(expected_file)?;
    // catr will have an additional carriage return.
    expected = expected + "\n";
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}
