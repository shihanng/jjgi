use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn basic() {
    let mut cmd = cargo_bin_cmd!("gi");

    cmd.args(["--", "./tests/scripts/basic.sh"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("test");
}
