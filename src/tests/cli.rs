use super::*;

#[test]
fn parse_reads_query_and_flags() -> Result<()> {
    let cli = Cli::parse([
        "-q".to_owned(),
        "  13.x   Request Body ".to_owned(),
        "--verbose".to_owned(),
        "--update".to_owned(),
    ])?;

    assert_eq!(
        cli,
        Cli {
            query: "  13.x   Request Body ".to_owned(),
            verbose: true,
            update: true,
        }
    );
    Ok(())
}

#[test]
fn parse_accepts_equals_query() -> Result<()> {
    let cli = Cli::parse(["--query=request body".to_owned()])?;

    assert_eq!(cli.query, "request body");
    Ok(())
}

#[test]
fn normalized_query_removes_every_selected_version_token() {
    let cli = Cli {
        query: "  13.x   Request Body 13.x ".to_owned(),
        ..Cli::default()
    };

    assert_eq!(cli.normalized_query("13.x"), "request body");
}

#[test]
fn parse_rejects_missing_query_value() {
    let error = Cli::parse(["--query".to_owned()]).expect_err("query value must be required");

    assert_eq!(error.to_string(), "--query requires a value");
}

#[test]
fn parse_rejects_missing_short_query_value() {
    let error = Cli::parse(["-q".to_owned()]).expect_err("query value must be required");

    assert_eq!(error.to_string(), "-q requires a value");
}

#[test]
fn parse_rejects_unknown_arguments() {
    let error = Cli::parse(["--unknown".to_owned()]).expect_err("unknown flag must fail");

    assert_eq!(error.to_string(), "unknown argument: --unknown");
}
