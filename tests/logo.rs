use assert_cmd::prelude::*;
use std::process::Command;

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
