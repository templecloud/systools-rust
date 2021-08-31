use assert_cmd::Command;

#[test]
fn false_itest() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
