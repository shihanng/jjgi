use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn basic() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args(["--", "./tests/scripts/basic.sh"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("test")
        .stderr("");
}

#[test]
fn no_stdout() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args(["--", "./tests/scripts/no_stdout.sh"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("")
        .stderr("Done\n");
}

#[test]
fn no_stdout_use_stdin() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args([
        "--on-success-stdout=stdin",
        "--on-success-stderr=stderr",
        "--",
        "./tests/scripts/no_stdout.sh",
    ])
    .write_stdin("test")
    .assert()
    .success()
    .stdout("test")
    .stderr("Done\n");
}

#[test]
fn stdin_file() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args(["--stdin-file", "--", "cat", "-n", "{stdin_file}"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("     1\ttest")
        .stderr("");
}

#[test]
fn stdin_file_no_arg() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args(["--stdin-file", "--", "cat"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn err() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args(["--", "./tests/scripts/err.sh"])
        .write_stdin("test")
        .assert()
        .failure()
        .stdout("")
        .stderr("");
}

#[test]
fn err_stderr() {
    let mut cmd = cargo_bin_cmd!("jjgi");

    cmd.args(["--on-failure-stderr=stdout", "--", "./tests/scripts/err.sh"])
        .write_stdin("test")
        .assert()
        .failure()
        .stdout("")
        .stderr("Fail to process\n");
}
