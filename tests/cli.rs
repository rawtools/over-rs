use std::error::Error;

use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

#[test]

fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("over")?;

    cmd.assert().success();

    //.stdout("Hello, world!\n");

    Ok(())
}
