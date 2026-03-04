use dagger_sdk::{Container, Directory, Query};

/// Base Rust container with cargo caches.
pub fn rust_builder(client: &Query, source: Directory) -> Container {
    let cargo_registry = client.cache_volume("forge-sdk-rust-cargo-registry");
    let cargo_git = client.cache_volume("forge-sdk-rust-cargo-git");
    let cargo_target = client.cache_volume("forge-sdk-rust-cargo-target");

    client
        .container()
        .from("rust:1-bookworm")
        .with_mounted_directory("/build", source)
        .with_workdir("/build")
        .with_mounted_cache("/usr/local/cargo/registry", cargo_registry)
        .with_mounted_cache("/usr/local/cargo/git", cargo_git)
        .with_mounted_cache("/build/target", cargo_target)
}
