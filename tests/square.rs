use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn square() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("logo")?;

    cmd.arg("progs/square.logo");
    cmd.assert().success();

    Ok(())
}
