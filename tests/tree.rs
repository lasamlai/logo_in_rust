use assert_cmd::prelude::*;
use std::process::Command;

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
