//! Integration tests for stkd operations.

mod common;

use common::{run_git, run_stkd, run_stkd_success, TestContext};

#[test]
fn test_init_creates_stack_directory() {
    let ctx = TestContext::new();

    run_stkd_success(&ctx.path, &["init"]);

    // Verify .git/stkd directory exists with config and state files
    assert!(ctx.path.join(".git/stkd").exists());
    assert!(ctx.path.join(".git/stkd/config.json").exists());
    assert!(ctx.path.join(".git/stkd/state.json").exists());
}

#[test]
fn test_init_warns_if_already_initialized() {
    let ctx = TestContext::new();

    // First init should succeed
    let output1 = run_stkd(&ctx.path, &["init"]);
    assert!(output1.status.success());

    // Second init should succeed but show a warning
    let output2 = run_stkd(&ctx.path, &["init"]);
    assert!(output2.status.success());

    let stderr = String::from_utf8_lossy(&output2.stderr);
    let stdout = String::from_utf8_lossy(&output2.stdout);
    // Check that "already initialized" appears somewhere in the output
    assert!(
        stderr.contains("already initialized") || stdout.contains("already initialized"),
        "Expected warning about already initialized, got stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_create_branch() {
    let ctx = TestContext::with_stkd();

    run_stkd_success(&ctx.path, &["create", "feature/test"]);

    // Verify branch was created and is current
    assert!(ctx.branch_exists("feature/test"));
    assert_eq!(ctx.current_branch(), "feature/test");
}

#[test]
fn test_create_branch_with_parent() {
    let ctx = TestContext::with_stkd();

    // Create first feature branch
    run_stkd_success(&ctx.path, &["create", "feature/parent"]);
    ctx.commit_file("parent.txt", "parent content", "Add parent file");

    // Create child branch on top of parent
    run_stkd_success(&ctx.path, &["create", "feature/child"]);
    ctx.commit_file("child.txt", "child content", "Add child file");

    // Verify both branches exist
    assert!(ctx.branch_exists("feature/parent"));
    assert!(ctx.branch_exists("feature/child"));

    // Verify we're on the child branch
    assert_eq!(ctx.current_branch(), "feature/child");
}

#[test]
fn test_create_from_trunk() {
    let ctx = TestContext::with_stkd();

    // Create first branch
    run_stkd_success(&ctx.path, &["create", "feature/first"]);
    ctx.commit_file("first.txt", "first", "Add first");

    // Create second branch from trunk (not from first)
    run_stkd_success(&ctx.path, &["create", "--from-trunk", "feature/second"]);

    // Verify we're on second branch
    assert_eq!(ctx.current_branch(), "feature/second");
}

#[test]
fn test_navigation_up_down() {
    let ctx = TestContext::with_stkd();

    // Create a stack of branches
    run_stkd_success(&ctx.path, &["create", "feature/a"]);
    ctx.commit_file("a.txt", "a", "Add a");

    run_stkd_success(&ctx.path, &["create", "feature/b"]);
    ctx.commit_file("b.txt", "b", "Add b");

    run_stkd_success(&ctx.path, &["create", "feature/c"]);
    ctx.commit_file("c.txt", "c", "Add c");

    // We're at feature/c, go down
    run_stkd_success(&ctx.path, &["down"]);
    assert_eq!(ctx.current_branch(), "feature/b");

    run_stkd_success(&ctx.path, &["down"]);
    assert_eq!(ctx.current_branch(), "feature/a");

    // Go back up
    run_stkd_success(&ctx.path, &["up"]);
    assert_eq!(ctx.current_branch(), "feature/b");
}

#[test]
fn test_navigation_top_bottom() {
    let ctx = TestContext::with_stkd();

    // Create a stack of branches
    run_stkd_success(&ctx.path, &["create", "feature/a"]);
    run_stkd_success(&ctx.path, &["create", "feature/b"]);
    run_stkd_success(&ctx.path, &["create", "feature/c"]);

    // Go to bottom (feature/a)
    run_stkd_success(&ctx.path, &["bottom"]);
    assert_eq!(ctx.current_branch(), "feature/a");

    // Go to top (feature/c)
    run_stkd_success(&ctx.path, &["top"]);
    assert_eq!(ctx.current_branch(), "feature/c");
}

#[test]
fn test_log_shows_stack() {
    let ctx = TestContext::with_stkd();

    // Create a stack
    run_stkd_success(&ctx.path, &["create", "feature/a"]);
    run_stkd_success(&ctx.path, &["create", "feature/b"]);

    // Log should show the stack
    let output = run_stkd_success(&ctx.path, &["log"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("feature/a") || stdout.contains("feature/b"));
}

#[test]
fn test_status_command() {
    let ctx = TestContext::with_stkd();

    run_stkd_success(&ctx.path, &["create", "feature/test"]);

    let output = run_stkd_success(&ctx.path, &["status"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Status should mention current branch or stack info
    assert!(!stdout.is_empty());
}

#[test]
fn test_delete_branch() {
    let ctx = TestContext::with_stkd();

    // Create a branch
    run_stkd_success(&ctx.path, &["create", "feature/to-delete"]);
    assert!(ctx.branch_exists("feature/to-delete"));

    // Go back to main
    run_git(&ctx.path, &["checkout", "main"]);

    // Delete the branch
    run_stkd_success(&ctx.path, &["delete", "feature/to-delete", "--force"]);

    // Branch should be gone
    assert!(!ctx.branch_exists("feature/to-delete"));
}

#[test]
fn test_rename_branch() {
    let ctx = TestContext::with_stkd();

    run_stkd_success(&ctx.path, &["create", "feature/old-name"]);
    assert!(ctx.branch_exists("feature/old-name"));

    run_stkd_success(&ctx.path, &["rename", "feature/new-name"]);

    assert!(!ctx.branch_exists("feature/old-name"));
    assert!(ctx.branch_exists("feature/new-name"));
    assert_eq!(ctx.current_branch(), "feature/new-name");
}

#[test]
fn test_checkout_by_name() {
    let ctx = TestContext::with_stkd();

    run_stkd_success(&ctx.path, &["create", "feature/a"]);
    run_stkd_success(&ctx.path, &["create", "feature/b"]);

    // Go to a
    run_stkd_success(&ctx.path, &["checkout", "feature/a"]);
    assert_eq!(ctx.current_branch(), "feature/a");

    // Go to b
    run_stkd_success(&ctx.path, &["checkout", "feature/b"]);
    assert_eq!(ctx.current_branch(), "feature/b");
}

#[test]
fn test_fails_without_init() {
    let ctx = TestContext::new();
    // Don't run init

    // Stack commands should fail without init
    let output = run_stkd(&ctx.path, &["create", "feature/test"]);
    assert!(!output.status.success());
}

#[test]
fn test_modify_amends_commit() {
    let ctx = TestContext::with_stkd();

    run_stkd_success(&ctx.path, &["create", "feature/test"]);
    ctx.commit_file("test.txt", "initial", "Initial commit");

    let initial_sha = ctx.head_sha();

    // Modify the file
    std::fs::write(ctx.path.join("test.txt"), "modified").unwrap();
    run_git(&ctx.path, &["add", "test.txt"]);

    // Amend the commit using "modify" command
    run_stkd_success(&ctx.path, &["modify"]);

    // SHA should have changed
    assert_ne!(ctx.head_sha(), initial_sha);
}

#[test]
fn test_version_command() {
    let ctx = TestContext::new();

    let output = run_stkd_success(&ctx.path, &["--version"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain version number
    assert!(stdout.contains("gt") || stdout.contains("0."));
}

#[test]
fn test_help_command() {
    let ctx = TestContext::new();

    let output = run_stkd_success(&ctx.path, &["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain usage information
    assert!(stdout.contains("Usage") || stdout.contains("usage"));
}
