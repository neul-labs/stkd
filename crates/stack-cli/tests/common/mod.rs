//! Common test utilities for integration tests.

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

/// Test context that provides a temporary git repository for testing.
pub struct TestContext {
    /// Temporary directory holding the repository (kept for RAII cleanup)
    #[allow(dead_code)]
    dir: TempDir,
    /// Path to the repository root
    pub path: PathBuf,
}

impl TestContext {
    /// Create a new test context with an initialized git repository.
    pub fn new() -> Self {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let path = dir.path().to_path_buf();

        // Initialize git repo
        run_git(&path, &["init"]);

        // Configure git user for commits
        run_git(&path, &["config", "user.email", "test@example.com"]);
        run_git(&path, &["config", "user.name", "Test User"]);

        // Create initial commit on main
        std::fs::write(path.join("README.md"), "# Test Repository\n")
            .expect("Failed to write README");
        run_git(&path, &["add", "."]);
        run_git(&path, &["commit", "-m", "Initial commit"]);

        Self { dir, path }
    }

    /// Create a new test context with Stack initialized.
    pub fn with_stack() -> Self {
        let ctx = Self::new();

        // Initialize stack
        run_stack(&ctx.path, &["init"]);

        ctx
    }

    /// Get the current branch name.
    pub fn current_branch(&self) -> String {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(&self.path)
            .output()
            .expect("Failed to get current branch");

        String::from_utf8(output.stdout)
            .expect("Invalid UTF-8")
            .trim()
            .to_string()
    }

    /// Create a file and commit it.
    pub fn commit_file(&self, filename: &str, content: &str, message: &str) {
        std::fs::write(self.path.join(filename), content)
            .expect("Failed to write file");
        run_git(&self.path, &["add", filename]);
        run_git(&self.path, &["commit", "-m", message]);
    }

    /// Get all local branch names.
    pub fn branches(&self) -> Vec<String> {
        let output = Command::new("git")
            .args(["branch", "--format=%(refname:short)"])
            .current_dir(&self.path)
            .output()
            .expect("Failed to list branches");

        String::from_utf8(output.stdout)
            .expect("Invalid UTF-8")
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Check if a branch exists.
    pub fn branch_exists(&self, name: &str) -> bool {
        self.branches().contains(&name.to_string())
    }

    /// Get commit count on current branch.
    pub fn commit_count(&self) -> usize {
        let output = Command::new("git")
            .args(["rev-list", "--count", "HEAD"])
            .current_dir(&self.path)
            .output()
            .expect("Failed to count commits");

        String::from_utf8(output.stdout)
            .expect("Invalid UTF-8")
            .trim()
            .parse()
            .unwrap_or(0)
    }

    /// Get the HEAD commit SHA.
    pub fn head_sha(&self) -> String {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&self.path)
            .output()
            .expect("Failed to get HEAD SHA");

        String::from_utf8(output.stdout)
            .expect("Invalid UTF-8")
            .trim()
            .to_string()
    }

    /// Check if working tree is clean.
    pub fn is_clean(&self) -> bool {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&self.path)
            .output()
            .expect("Failed to get git status");

        output.stdout.is_empty()
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Run a git command in the given directory.
pub fn run_git(dir: &Path, args: &[&str]) -> std::process::Output {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to run git {:?}: {}", args, e));

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("git {:?} failed: {}", args, stderr);
    }

    output
}

/// Run a stack command in the given directory.
/// Returns the output regardless of success/failure.
pub fn run_stack(dir: &Path, args: &[&str]) -> std::process::Output {
    // Build the path to the stack binary
    let stack_bin = find_stack_binary();

    Command::new(&stack_bin)
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to run stack {:?}: {} (binary: {})", args, e, stack_bin))
}

/// Find the gt binary for testing.
fn find_stack_binary() -> String {
    // First try the CARGO_BIN_EXE environment variable (set by cargo test)
    if let Ok(bin) = std::env::var("CARGO_BIN_EXE_gt") {
        return bin;
    }

    // CARGO_MANIFEST_DIR points to crates/stack-cli, so go up to workspace root
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = std::path::Path::new(manifest_dir)
        .parent() // crates/
        .and_then(|p| p.parent()) // workspace root
        .expect("Could not find workspace root");

    // Try debug build
    let debug_bin = workspace_root.join("target/debug/gt");
    if debug_bin.exists() {
        return debug_bin.to_string_lossy().to_string();
    }

    // Try release build
    let release_bin = workspace_root.join("target/release/gt");
    if release_bin.exists() {
        return release_bin.to_string_lossy().to_string();
    }

    // Fall back to hoping it's in PATH
    "gt".to_string()
}

/// Run a stack command and expect it to succeed.
pub fn run_stack_success(dir: &Path, args: &[&str]) -> std::process::Output {
    let output = run_stack(dir, args);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!(
            "stack {:?} failed:\nstdout: {}\nstderr: {}",
            args, stdout, stderr
        );
    }

    output
}

/// Run a stack command and expect it to fail.
#[allow(dead_code)]
pub fn run_stack_failure(dir: &Path, args: &[&str]) -> std::process::Output {
    let output = run_stack(dir, args);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("stack {:?} should have failed but succeeded:\n{}", args, stdout);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creates_valid_repo() {
        let ctx = TestContext::new();
        assert!(ctx.path.exists());
        assert!(ctx.path.join(".git").exists());
        assert_eq!(ctx.current_branch(), "master");
        assert!(ctx.is_clean());
    }

    #[test]
    fn test_context_commit_file() {
        let ctx = TestContext::new();
        let initial_count = ctx.commit_count();

        ctx.commit_file("test.txt", "Hello", "Add test file");

        assert_eq!(ctx.commit_count(), initial_count + 1);
        assert!(ctx.path.join("test.txt").exists());
    }
}
