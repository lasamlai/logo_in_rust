use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn case1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/case1.logo");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("10\n404\n"));

    Ok(())
}

#[test]
fn case2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/case2.logo");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("10\n".repeat(10)));

    Ok(())
}

#[test]
fn case3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/case3.logo");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Logo\nLogo\n"));

    Ok(())
}

#[test]
fn fern() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/fern.logo");
    cmd.assert().success();

    Command::new("diff")
        .arg("./image.svg")
        .arg("./progs/fern.svg")
        .assert()
        .success();

    Ok(())
}

#[test]
fn logo() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/logo.logo");
    cmd.assert().success();

    Command::new("diff")
        .arg("./image.svg")
        .arg("./progs/logo.svg")
        .assert()
        .success();

    Ok(())
}

#[test]
fn square() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/square.logo");
    cmd.assert().success();

    Ok(())
}

#[test]
fn star() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/star.logo");
    cmd.assert().success();

    Command::new("diff")
        .arg("./image.svg")
        .arg("./progs/star.svg")
        .assert()
        .success();

    Ok(())
}

#[test]
fn tree() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/tree.logo");
    cmd.assert().success();

    Command::new("diff")
        .arg("./image.svg")
        .arg("./progs/tree.svg")
        .assert()
        .success();

    Ok(())
}
