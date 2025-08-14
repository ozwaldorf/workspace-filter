//! # Workspace Filter Build
//!
//! Build script end of the workspace filter framework.
//! For more information see the [workspace-filter](https://docs.rs/workspace-filter) documentation.

use cargo_metadata::MetadataCommand;

/// Build and output an environment variable `WORKSPACE_FILTER` to the crate.
///
/// Schedules build script to rerun if Cargo.lock changes.
/// Must be used from inside a crate's build script.
///
/// # Errors
///
/// If the workspace couldn't be loaded for any reason.
pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = MetadataCommand::new().exec()?;

    println!(
        "cargo:rerun-if-changed={}/Cargo.lock",
        metadata.workspace_root
    );

    let filter = metadata
        .workspace_packages()
        .into_iter()
        .map(|p| format!("{}={{level}}", p.name))
        .collect::<Vec<_>>()
        .join(",");
    println!("cargo:rustc-env=WORKSPACE_FILTER={filter}");

    Ok(())
}
