use dagger_sdk::{Directory, Query};
use eyre::WrapErr;

use crate::containers::rust_builder;

/// Run cargo tests.
pub async fn run(client: &Query, source: Directory) -> eyre::Result<String> {
    let output = rust_builder(client, source)
        .with_exec(vec!["cargo", "test"])
        .with_exec(vec!["sh", "-c", "echo 'test: all tests passed'"])
        .stdout()
        .await
        .wrap_err("test failed")?;

    Ok(output)
}
