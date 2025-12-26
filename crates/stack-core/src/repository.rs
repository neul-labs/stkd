//! Repository wrapper for Stack operations.
//!
//! This module provides the main entry point for Stack operations. The [`Repository`]
//! struct wraps a Git repository and adds stacked branch management functionality.
//!
//! # Overview
//!
//! A Stack-enabled repository maintains:
//! - Branch tracking metadata (parent-child relationships)
//! - Configuration (trunk branch, provider settings)
//! - Merge request associations
//!
//! # Example
//!
//! ```rust,ignore
//! use stack_core::Repository;
//!
//! // Open an existing Stack-enabled repository
//! let repo = Repository::open(".")?;
//!
//! // Create a new branch on top of current
//! let branch = repo.create_branch("feature/login")?;
//! println!("Created {} with parent {}", branch.name, branch.parent);
//!
//! // Navigate the stack
//! repo.up(1)?;  // Move to child branch
//! repo.down(1)?; // Move to parent branch
//! repo.top()?;   // Go to stack tip
//! repo.bottom()?; // Go to stack root
//! ```
//!
//! # Initialization
//!
//! Before using Stack features, a repository must be initialized:
//!
//! ```rust,ignore
//! use stack_core::Repository;
//!
//! // Initialize Stack in an existing Git repo
//! let repo = Repository::init(".")?;
//! println!("Trunk branch: {}", repo.trunk());
//! ```

use std::path::Path;

use git2::{BranchType, Repository as GitRepo};
use tracing::{debug, info};

use crate::branch::BranchInfo;
use crate::config::StackConfig;
use crate::dag::BranchGraph;
use crate::stack::Stack;
use crate::storage::Storage;
use crate::{Error, Result};

/// A Stack-enabled Git repository.
///
/// This is the main entry point for Stack operations. It wraps a Git repository
/// and provides methods for:
///
/// - Creating and managing stacked branches
/// - Navigating between branches in a stack
/// - Tracking branch metadata and PR associations
/// - Loading and modifying stack configuration
///
/// # Thread Safety
///
/// This struct is not thread-safe. Each thread should open its own instance.
///
/// # Example
///
/// ```rust,ignore
/// use stack_core::Repository;
///
/// let repo = Repository::open(".")?;
///
/// // Check current state
/// println!("Current branch: {:?}", repo.current_branch()?);
/// println!("Trunk: {}", repo.trunk());
/// println!("Clean working tree: {}", repo.is_clean()?);
/// ```
pub struct Repository {
    /// Underlying Git repository
    git: GitRepo,
    /// Stack storage
    storage: Storage,
    /// Cached configuration
    config: StackConfig,
}

impl Repository {
    /// Open a Stack-enabled repository.
    ///
    /// Discovers the Git repository from the given path and loads Stack metadata.
    /// The path can be anywhere within the repository working tree.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the repository or any subdirectory within it
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No Git repository is found at or above the path
    /// - The repository is not initialized for Stack (run `gt init` first)
    /// - Stack metadata is corrupted
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let repo = Repository::open(".")?;
    /// let repo = Repository::open("/path/to/repo/src/components")?; // Works from subdirectory
    /// ```
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let git = GitRepo::discover(path.as_ref()).map_err(|_| Error::NotARepository)?;

        let git_dir = git.path();
        let storage = Storage::open(git_dir)?;

        if !storage.is_initialized() {
            return Err(Error::NotInitialized);
        }

        let config = storage.load_config()?;

        Ok(Self { git, storage, config })
    }

    /// Initialize Stack in an existing Git repository.
    ///
    /// Creates the Stack metadata directory (`.git/stack/`) and auto-detects
    /// configuration settings like the trunk branch name.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the repository or any subdirectory within it
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No Git repository is found
    /// - Stack is already initialized
    /// - Cannot create metadata directory
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Initialize Stack in the current directory
    /// let repo = Repository::init(".")?;
    /// println!("Initialized with trunk: {}", repo.trunk());
    /// ```
    pub fn init(path: impl AsRef<Path>) -> Result<Self> {
        let git = GitRepo::discover(path.as_ref()).map_err(|_| Error::NotARepository)?;

        let git_dir = git.path();
        let storage = Storage::init(git_dir)?;

        // Detect and save configuration
        let config = StackConfig::detect(&git);
        storage.save_config(&config)?;

        info!("Initialized Stack with trunk: {}", config.trunk);

        Ok(Self { git, storage, config })
    }

    /// Get the underlying Git repository
    pub fn git(&self) -> &GitRepo {
        &self.git
    }

    /// Get the storage
    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    /// Get the configuration
    pub fn config(&self) -> &StackConfig {
        &self.config
    }

    /// Get the trunk branch name
    pub fn trunk(&self) -> &str {
        &self.config.trunk
    }

    /// Get the current branch name
    pub fn current_branch(&self) -> Result<Option<String>> {
        let head = self.git.head()?;
        if head.is_branch() {
            Ok(head.shorthand().map(String::from))
        } else {
            Ok(None)
        }
    }

    /// Check if we're on the trunk branch
    pub fn is_on_trunk(&self) -> Result<bool> {
        Ok(self.current_branch()? == Some(self.trunk().to_string()))
    }

    /// Check if working directory is clean
    pub fn is_clean(&self) -> Result<bool> {
        let statuses = self.git.statuses(None)?;
        Ok(statuses.is_empty())
    }

    /// Ensure working directory is clean
    pub fn ensure_clean(&self) -> Result<()> {
        if !self.is_clean()? {
            return Err(Error::UncommittedChanges);
        }
        Ok(())
    }

    // ========================================================================
    // Branch Operations
    // ========================================================================

    /// Create a new branch on top of the current one.
    ///
    /// Creates a new Git branch at the current HEAD and sets up Stack tracking
    /// with the current branch as the parent. The new branch is automatically
    /// checked out.
    ///
    /// # Arguments
    ///
    /// * `name` - Name for the new branch (e.g., "feature/login")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Not currently on a branch (detached HEAD)
    /// - Branch name is invalid
    /// - Branch already exists
    /// - Git operations fail
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let repo = Repository::open(".")?;
    ///
    /// // Create a feature branch
    /// let branch = repo.create_branch("feature/auth")?;
    /// println!("Created {} with parent {}", branch.name, branch.parent);
    ///
    /// // Create a child of the feature branch
    /// let child = repo.create_branch("feature/auth-tests")?;
    /// assert_eq!(child.parent, "feature/auth");
    /// ```
    pub fn create_branch(&self, name: &str) -> Result<BranchInfo> {
        // Validate branch name
        if name.is_empty() || name.contains("..") {
            return Err(Error::InvalidBranchName(name.to_string()));
        }

        // Check if branch already exists
        if self.git.find_branch(name, BranchType::Local).is_ok() {
            return Err(Error::BranchExists(name.to_string()));
        }

        // Get current branch as parent
        let parent = self
            .current_branch()?
            .ok_or_else(|| Error::other("Not on a branch"))?;

        // Get current HEAD commit
        let head = self.git.head()?.peel_to_commit()?;
        let head_id = head.id().to_string();

        // Create the branch
        self.git.branch(name, &head, false)?;

        // Checkout the new branch
        self.git.set_head(&format!("refs/heads/{}", name))?;
        self.git.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;

        // Create and save branch info
        let mut info = BranchInfo::new(name, &parent);
        info.base_commit = Some(head_id.clone());
        info.head_commit = Some(head_id);

        self.storage.save_branch(&info)?;

        // Update parent's children
        if self.storage.is_tracked(&parent) {
            self.storage.update_branch(&parent, |p| {
                p.add_child(name);
            })?;
        }

        info!("Created branch {} on top of {}", name, parent);

        Ok(info)
    }

    /// Track an existing branch
    pub fn track_branch(&self, name: &str) -> Result<BranchInfo> {
        // Verify branch exists
        let branch = self
            .git
            .find_branch(name, BranchType::Local)
            .map_err(|_| Error::BranchNotFound(name.to_string()))?;

        // Check if already tracked
        if self.storage.is_tracked(name) {
            return self
                .storage
                .load_branch(name)?
                .ok_or_else(|| Error::other("Branch info missing"));
        }

        // Get branch commit
        let commit = branch.get().peel_to_commit()?;

        // Default parent to trunk
        let parent = self.trunk().to_string();

        let mut info = BranchInfo::new(name, &parent);
        info.head_commit = Some(commit.id().to_string());

        self.storage.save_branch(&info)?;

        info!("Now tracking branch {}", name);

        Ok(info)
    }

    /// Untrack a branch (stop managing it, don't delete)
    pub fn untrack_branch(&self, name: &str) -> Result<()> {
        let info = self
            .storage
            .load_branch(name)?
            .ok_or_else(|| Error::BranchNotTracked(name.to_string()))?;

        // Remove from parent's children
        if self.storage.is_tracked(&info.parent) {
            self.storage.update_branch(&info.parent, |p| {
                p.remove_child(name);
            })?;
        }

        // Update children to point to our parent
        for child in &info.children {
            if self.storage.is_tracked(child) {
                self.storage.update_branch(child, |c| {
                    c.parent = info.parent.clone();
                })?;
            }
        }

        self.storage.delete_branch(name)?;

        info!("Untracked branch {}", name);

        Ok(())
    }

    /// Delete a branch
    pub fn delete_branch(&self, name: &str, force: bool) -> Result<()> {
        if name == self.trunk() {
            return Err(Error::CannotOperateOnTrunk(name.to_string()));
        }

        // Get branch info before deletion
        let info = self.storage.load_branch(name)?;

        // Check if branch has children
        if let Some(ref info) = info {
            if !info.children.is_empty() && !force {
                return Err(Error::other(format!(
                    "Branch {} has children: {}. Use --force to delete anyway.",
                    name,
                    info.children.join(", ")
                )));
            }
        }

        // Delete Git branch
        let mut branch = self.git.find_branch(name, BranchType::Local)?;
        branch.delete()?;

        // Untrack if tracked
        if info.is_some() {
            self.untrack_branch(name)?;
        }

        info!("Deleted branch {}", name);

        Ok(())
    }

    /// Rename the current branch
    pub fn rename_branch(&self, new_name: &str) -> Result<BranchInfo> {
        let current = self
            .current_branch()?
            .ok_or_else(|| Error::other("Not on a branch"))?;

        if current == self.trunk() {
            return Err(Error::CannotOperateOnTrunk(current));
        }

        // Rename Git branch
        let mut branch = self.git.find_branch(&current, BranchType::Local)?;
        branch.rename(new_name, false)?;

        // Update tracking
        if let Some(mut info) = self.storage.load_branch(&current)? {
            // Delete old
            self.storage.delete_branch(&current)?;

            // Update parent's children reference
            if self.storage.is_tracked(&info.parent) {
                self.storage.update_branch(&info.parent, |p| {
                    p.remove_child(&current);
                    p.add_child(new_name);
                })?;
            }

            // Update children's parent reference
            for child in &info.children {
                if self.storage.is_tracked(child) {
                    self.storage.update_branch(child, |c| {
                        c.parent = new_name.to_string();
                    })?;
                }
            }

            // Save with new name
            info.name = new_name.to_string();
            self.storage.save_branch(&info)?;

            info!("Renamed branch {} to {}", current, new_name);

            Ok(info)
        } else {
            Err(Error::BranchNotTracked(current))
        }
    }

    // ========================================================================
    // Navigation
    // ========================================================================

    /// Move up the stack (to child branch)
    pub fn up(&self, steps: usize) -> Result<String> {
        let current = self
            .current_branch()?
            .ok_or_else(|| Error::other("Not on a branch"))?;

        let graph = self.load_graph()?;
        let stack = Stack::from_graph(&graph, &current, Some(&current));

        let mut target = current.clone();
        for _ in 0..steps {
            if let Some(entry) = stack.iter().find(|e| e.name() == target).and_then(|_| stack.up()) {
                target = entry.name().to_string();
            } else {
                return Err(Error::other("Already at stack tip"));
            }
        }

        self.checkout(&target)?;
        Ok(target)
    }

    /// Move down the stack (to parent branch)
    pub fn down(&self, steps: usize) -> Result<String> {
        let current = self
            .current_branch()?
            .ok_or_else(|| Error::other("Not on a branch"))?;

        let graph = self.load_graph()?;
        let stack = Stack::from_graph(&graph, &current, Some(&current));

        let mut target = current.clone();
        for _ in 0..steps {
            if let Some(entry) = stack.iter().find(|e| e.name() == target).and_then(|_| stack.down()) {
                target = entry.name().to_string();
            } else {
                return Err(Error::other("Already at stack root"));
            }
        }

        self.checkout(&target)?;
        Ok(target)
    }

    /// Go to stack tip
    pub fn top(&self) -> Result<String> {
        let current = self
            .current_branch()?
            .ok_or_else(|| Error::other("Not on a branch"))?;

        let graph = self.load_graph()?;
        let stack = Stack::from_graph(&graph, &current, Some(&current));

        if let Some(tip) = stack.tip() {
            self.checkout(tip.name())?;
            Ok(tip.name().to_string())
        } else {
            Err(Error::other("Stack is empty"))
        }
    }

    /// Go to stack bottom
    pub fn bottom(&self) -> Result<String> {
        let current = self
            .current_branch()?
            .ok_or_else(|| Error::other("Not on a branch"))?;

        let graph = self.load_graph()?;
        let stack = Stack::from_graph(&graph, &current, Some(&current));

        if let Some(root) = stack.root() {
            self.checkout(root.name())?;
            Ok(root.name().to_string())
        } else {
            Err(Error::other("Stack is empty"))
        }
    }

    /// Checkout a branch
    pub fn checkout(&self, name: &str) -> Result<()> {
        debug!("Checking out {}", name);

        self.git.set_head(&format!("refs/heads/{}", name))?;
        self.git.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;

        Ok(())
    }

    // ========================================================================
    // Stack Operations
    // ========================================================================

    /// Load the branch dependency graph.
    ///
    /// Returns a [`BranchGraph`] containing all tracked branches and their
    /// parent-child relationships. This is useful for visualizing the stack
    /// structure or determining restack order.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let repo = Repository::open(".")?;
    /// let graph = repo.load_graph()?;
    ///
    /// // Get all branches in the current stack
    /// let current = repo.current_branch()?.unwrap();
    /// let stack = graph.stack(&current);
    /// for branch in stack {
    ///     println!("{}", branch);
    /// }
    /// ```
    pub fn load_graph(&self) -> Result<BranchGraph> {
        let branches = self.storage.list_branches()?;
        Ok(BranchGraph::from_branches(branches, self.trunk()))
    }

    /// Get the current stack
    pub fn current_stack(&self) -> Result<Stack<'_>> {
        let current = self.current_branch()?;
        let graph = self.load_graph()?;

        // If on trunk or untracked branch, find first tracked child
        let center = if let Some(ref branch) = current {
            if self.storage.is_tracked(branch) {
                branch.clone()
            } else {
                // Find a tracked branch
                graph
                    .all_branches()
                    .next()
                    .map(|b| b.name.clone())
                    .ok_or_else(|| Error::other("No tracked branches"))?
            }
        } else {
            return Err(Error::other("Not on a branch"));
        };

        // Need to reload graph to get proper lifetime
        let graph = self.load_graph()?;

        // This is a simplification - in practice we'd need to handle lifetimes better
        // For now, return an empty stack indication
        Ok(Stack::from_graph(
            // This won't work due to lifetime issues - would need refactoring
            // For the prototype, we'll handle this differently in the CLI
            Box::leak(Box::new(graph)),
            &center,
            current.as_deref(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_repo() -> (TempDir, Repository) {
        let dir = TempDir::new().unwrap();

        // Initialize Git repo
        let git = GitRepo::init(dir.path()).unwrap();

        // Create initial commit
        {
            let sig = git.signature().unwrap();
            let tree_id = git.index().unwrap().write_tree().unwrap();
            let tree = git.find_tree(tree_id).unwrap();
            git.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
                .unwrap();
        }

        // Initialize Stack
        let repo = Repository::init(dir.path()).unwrap();

        (dir, repo)
    }

    #[test]
    fn test_init() {
        let (dir, _repo) = setup_repo();
        assert!(dir.path().join(".git/stack").exists());
    }

    #[test]
    fn test_create_branch() {
        let (_dir, repo) = setup_repo();

        let info = repo.create_branch("feature/test").unwrap();
        assert_eq!(info.name, "feature/test");
        // Parent is the trunk branch (may be main or master depending on git config)
        assert_eq!(info.parent, repo.trunk());
    }

    #[test]
    fn test_track_branch() {
        let (_dir, repo) = setup_repo();

        // Create a branch without tracking
        repo.git().branch("untracked", &repo.git().head().unwrap().peel_to_commit().unwrap(), false).unwrap();

        let info = repo.track_branch("untracked").unwrap();
        assert_eq!(info.name, "untracked");
    }
}
