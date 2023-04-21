use assert_cmd::prelude::*;
use std::process::Command;

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
