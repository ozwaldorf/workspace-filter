#![cfg(test)]

use workspace_filter::workspace_filter;

#[test]
fn workspace_only() {
    assert_eq!(
        workspace_filter!("info"),
        "test=info,workspace-filter=info,workspace-filter-build=info"
    );
}

#[test]
fn with_default() {
    assert_eq!(
        workspace_filter!("debug", "info"),
        "info,test=debug,workspace-filter=debug,workspace-filter-build=debug"
    );
}

#[test]
fn with_default_using_level() {
    assert_eq!(
        workspace_filter!("debug", "info,my-bin={level}"),
        "info,my-bin=debug,test=debug,workspace-filter=debug,workspace-filter-build=debug"
    );
}
