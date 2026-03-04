use dagger_sdk::{Directory, Query};
use eyre::WrapErr;

use crate::containers::rust_builder;

/// Run clippy and format checks.
pub async fn run(client: &Query, source: Directory) -> eyre::Result<String> {
    let output = rust_builder(client, source)
        .with_exec(vec!["cargo", "clippy", "--", "-D", "warnings"])
        .with_exec(vec!["cargo", "fmt", "--check"])
        .with_exec(vec!["sh", "-c", "echo 'check: clippy + fmt passed'"])
        .stdout()
        .await
        .wrap_err("check failed")?;

    Ok(output)
}
