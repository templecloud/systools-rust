use assert_cmd::Command;

#[test]
fn test_falser_ok() {
    let mut cmd = Command::cargo_bin("falser").unwrap();
    cmd.assert().failure();
}
