use assert_cmd::prelude::*;
use std::process::Command;

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
