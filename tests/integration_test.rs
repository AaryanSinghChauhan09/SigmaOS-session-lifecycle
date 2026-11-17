// Integration test for SigmaOS-session-lifecycle
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use session_lifecycle as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
