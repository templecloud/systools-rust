use assert_cmd::Command;

#[test]
fn test_truer_ok() {
    let mut cmd = Command::cargo_bin("truer").unwrap();
    cmd.assert().success();
}
