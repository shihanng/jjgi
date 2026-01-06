use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn basic() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args(["--", "./tests/scripts/basic.sh"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("test");
}

#[test]
fn no_stdout() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args(["--", "./tests/scripts/no_stdout.sh"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("Done\n");
}

#[test]
fn no_stdout_use_stdin() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args([
        "--on-success-stdout=std-in",
        "--",
        "./tests/scripts/no_stdout.sh",
    ])
    .write_stdin("test")
    .assert()
    .success()
    .stdout("test");
}
