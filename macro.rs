#![doc = include_str!("README.md")]

/// Construct a new filter with all crates in the workspace set to a given level, optionally
/// including a default string. See module documentation for more info.
#[macro_export]
macro_rules! workspace_filter {
    ($level:expr) => {
        format!(env!("WORKSPACE_FILTER"), level = $level)
    };
    ($level:expr, $etc:expr) => {
        format!(
            concat!(concat!($etc, ","), env!("WORKSPACE_FILTER")),
            level = $level
        )
    };
}
