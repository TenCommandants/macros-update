use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_cli_apply() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfs")?;

    cmd.arg("apply");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Apply"));

    Ok(())
}

// #[test]
// fn test_cli_materialize() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("gfs")?;

//     let time_str = "2020-01-01T00:00:00Z";

//     cmd.arg("materialize").arg(time_str);
//     cmd.assert().success().stdout(predicate::str::contains(
//         "Materialize: ".to_owned() + time_str,
//     ));

//     Ok(())
// }

#[test]
fn test_cli_serve() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfs")?;

    cmd.arg("serve");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Serve"));

    Ok(())
}
