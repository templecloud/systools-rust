use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args_error() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn test_args_ok() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("some input").assert().success();

    cmd = Command::cargo_bin("echor")?;
    cmd.arg("some input")
        .assert()
        .success()
        .stdout("some input\n");

    cmd = Command::cargo_bin("echor")?;
    cmd.args(["some  more  input"])
        .assert()
        .success()
        .stdout("some  more  input\n");

    Ok(())
}

#[test]
fn test_n_opt_ok() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("-n")
        .arg("some input")
        .assert()
        .success()
        .stdout("some input");

    cmd = Command::cargo_bin("echor")?;
    cmd.args(["-n", "some  more  input"])
        .assert()
        .success()
        .stdout("some  more  input");
    Ok(())
}

#[test]
fn test_version_opt_ok() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("--version")
        .assert()
        .success()
        .stdout("echor 0.1.0\n");

    Ok(())
}

#[test]
fn test_h_opt_ok() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("-h").assert().success().stdout(predicate::ne(""));
    Ok(())
}
