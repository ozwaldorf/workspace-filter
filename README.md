# Workspace Filter

Simple framework to compile env filters for all packages in a workspace.

## Setup

First, add the two dependencies to your crate:

```toml
[dependencies]
workspace-filter = "0.1"

[build-dependencies]
workspace-filter-build = "0.1"
```

Then, call the build function in your build.rs file:

```rust,ignore
fn main() -> Result<(), Box<dyn std::error::Error> {
    // NOTE: This will rerun the entire build script if Cargo.lock changes
    workspace_filter_build::build()?;
}
```

The macro can then be used in your crate, with a few example use-cases shown below:

#### Simple Usage

If you only care to set the filter for your workspace dependencies:

```rust,ignore
let filter = workspace_filter!("debug");
let env_filter = EnvFilter::builder().parse_lossy(filter);
```

Manual filters can be included, for example to make workspace crates at debug and everything else at info:
```rust,ignore
let filter = workspace_filter!("debug", "info");
let env_filter = EnvFilter::builder().parse_lossy(filter);
```

In the catchall argument, `{level}` can be also used anywhere to substitute the given workspace level.

```rust,ignore
let filter = workspace_filter!("debug", "trace,my-bin={level}");
let env_filter = EnvFilter::builder().parse_lossy(filter);
```

#### Verbosity level Example

Example that overrides with `RUST_LOG`, or falls back to increasing the
verbosity for the workspace and other deps depending on a given number

```rust,ignore
let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
    // Fallback which is directed by the verbosity flag
    match args.verbose {
        0 => "info".into(),
        1 => workspace_filter!("debug", "info"),
        2 => workspace_filter!("trace", "info"),
        3 => workspace_filter!("trace", "debug"),
        _ => "trace".into(),
    }
});
let env_filter = EnvFilter::builder().parse_lossy(filter);
```


