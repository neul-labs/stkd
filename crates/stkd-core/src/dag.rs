//! Directed Acyclic Graph (DAG) for branch dependencies.
//!
//! This module provides the [`BranchGraph`] struct which manages the dependency
//! relationships between branches in a stacked diffs workflow. It tracks which
//! branches depend on which others and enables traversal and manipulation of
//! the dependency tree.
//!
//! # Why a DAG?
//!
//! In stacked diffs, branches form a directed acyclic graph:
//! - **Directed**: Each branch points to its parent (the branch it's based on)
//! - **Acyclic**: No branch can depend on itself, directly or indirectly
//!
//! ```text
//!                main (trunk)
//!                 /    \
//!             auth   logging    ← Two root branches
//!              |        |
//!           auth-ui  log-fmt    ← Children
//!              |
//!           auth-api            ← Grandchild
//! ```
//!
//! # Core Operations
//!
//! - **Traversal**: Find parents, children, ancestors, descendants
//! - **Stack assembly**: Get all branches in a linear stack
//! - **Topological sorting**: Order branches so parents come before children
//! - **Cycle detection**: Ensure the graph remains acyclic
//! - **Restack detection**: Find branches needing rebasing
//!
//! # Example
//!
//! ```rust
//! use stkd_core::dag::BranchGraph;
//! use stkd_core::branch::BranchInfo;
//!
//! // Create a graph with "main" as the trunk
//! let mut graph = BranchGraph::new("main");
//!
//! // Add branches
//! graph.add(BranchInfo::new("feature-1", "main"));
//! graph.add(BranchInfo::new("feature-2", "feature-1"));
//! graph.add(BranchInfo::new("feature-3", "feature-2"));
//!
//! // Query relationships
//! assert_eq!(graph.parent("feature-2"), Some("feature-1"));
//! assert_eq!(graph.children("feature-1"), vec!["feature-2"]);
//! assert_eq!(graph.depth("feature-3"), 3);
//!
//! // Get the full stack containing a branch
//! let stack = graph.stack("feature-2");
//! assert_eq!(stack, vec!["feature-1", "feature-2", "feature-3"]);
//! ```

use std::collections::{HashMap, HashSet, VecDeque};

use crate::branch::BranchInfo;
use crate::{Error, Result};

/// A directed acyclic graph of branch dependencies.
///
/// `BranchGraph` tracks all branches managed by Stack and their parent-child
/// relationships. It enables operations like:
///
/// - Finding the parent of a branch
/// - Getting all children of a branch
/// - Computing the full stack for a branch
/// - Detecting when branches need restacking
/// - Validating graph integrity (no cycles, no orphans)
///
/// # Trunk
///
/// The graph has a special "trunk" branch (usually `main` or `master`) which
/// serves as the root of all stacks. Branches whose parent is the trunk are
/// called "root branches" and form the base of their respective stacks.
///
/// # Thread Safety
///
/// `BranchGraph` is not thread-safe. For concurrent access, wrap it in
/// appropriate synchronization primitives.
#[derive(Debug, Default)]
pub struct BranchGraph {
    /// Map of branch name to branch info.
    branches: HashMap<String, BranchInfo>,
    /// Name of the trunk branch (e.g., "main" or "master").
    trunk: String,
}

impl BranchGraph {
    /// Creates a new empty graph with the specified trunk branch.
    ///
    /// # Arguments
    ///
    /// * `trunk` - The name of the trunk branch (e.g., "main", "master")
    ///
    /// # Example
    ///
    /// ```rust
    /// use stkd_core::dag::BranchGraph;
    ///
    /// let graph = BranchGraph::new("main");
    /// assert_eq!(graph.trunk(), "main");
    /// ```
    pub fn new(trunk: impl Into<String>) -> Self {
        Self {
            branches: HashMap::new(),
            trunk: trunk.into(),
        }
    }

    /// Creates a graph from a list of branches.
    ///
    /// This is typically used when loading branches from persistent storage.
    ///
    /// # Arguments
    ///
    /// * `branches` - List of branch info to populate the graph
    /// * `trunk` - The name of the trunk branch
    pub fn from_branches(branches: Vec<BranchInfo>, trunk: impl Into<String>) -> Self {
        let mut graph = Self::new(trunk);
        for branch in branches {
            graph.branches.insert(branch.name.clone(), branch);
        }
        graph
    }

    /// Returns the trunk branch name.
    pub fn trunk(&self) -> &str {
        &self.trunk
    }

    /// Returns `true` if the given name is the trunk branch.
    pub fn is_trunk(&self, name: &str) -> bool {
        name == self.trunk
    }

    /// Adds a branch to the graph.
    ///
    /// If a branch with the same name already exists, it is replaced.
    pub fn add(&mut self, info: BranchInfo) {
        self.branches.insert(info.name.clone(), info);
    }

    /// Removes a branch from the graph, returning it if it existed.
    ///
    /// **Warning**: Removing a branch that has children will leave those
    /// children as orphans. Use [`validate`](Self::validate) to check for orphans.
    pub fn remove(&mut self, name: &str) -> Option<BranchInfo> {
        self.branches.remove(name)
    }

    /// Returns a reference to a branch by name.
    pub fn get(&self, name: &str) -> Option<&BranchInfo> {
        self.branches.get(name)
    }

    /// Returns a mutable reference to a branch by name.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut BranchInfo> {
        self.branches.get_mut(name)
    }

    /// Returns `true` if the branch is tracked in this graph.
    pub fn contains(&self, name: &str) -> bool {
        self.branches.contains_key(name)
    }

    /// Returns an iterator over all tracked branches.
    pub fn all_branches(&self) -> impl Iterator<Item = &BranchInfo> {
        self.branches.values()
    }

    /// Returns the parent branch name for a given branch.
    ///
    /// Returns `None` if the branch is not tracked.
    pub fn parent(&self, name: &str) -> Option<&str> {
        self.branches.get(name).map(|b| b.parent.as_str())
    }

    /// Returns immediate children of a branch.
    ///
    /// These are branches whose parent is the given branch name.
    pub fn children(&self, name: &str) -> Vec<&str> {
        self.branches
            .values()
            .filter(|b| b.parent == name)
            .map(|b| b.name.as_str())
            .collect()
    }

    /// Returns all descendants of a branch (children, grandchildren, etc.).
    ///
    /// Uses breadth-first traversal, so closer descendants appear first.
    pub fn descendants(&self, name: &str) -> Vec<&str> {
        let mut result = vec![];
        let mut queue: VecDeque<&str> = self.children(name).into_iter().collect();

        while let Some(child) = queue.pop_front() {
            result.push(child);
            for grandchild in self.children(child) {
                queue.push_back(grandchild);
            }
        }

        result
    }

    /// Returns all ancestors of a branch (parent, grandparent, etc.) up to trunk.
    ///
    /// The result is ordered from immediate parent to furthest ancestor.
    /// Does not include the trunk branch.
    pub fn ancestors(&self, name: &str) -> Vec<&str> {
        let mut result = vec![];
        let mut current = name;

        while let Some(parent) = self.parent(current) {
            if self.is_trunk(parent) {
                break;
            }
            result.push(parent);
            current = parent;
        }

        result
    }

    /// Returns the complete stack containing a branch.
    ///
    /// This includes ancestors, the branch itself, and all descendants,
    /// ordered from root (closest to trunk) to tip (furthest from trunk).
    ///
    /// # Example
    ///
    /// ```rust
    /// use stkd_core::dag::BranchGraph;
    /// use stkd_core::branch::BranchInfo;
    ///
    /// let mut graph = BranchGraph::new("main");
    /// graph.add(BranchInfo::new("a", "main"));
    /// graph.add(BranchInfo::new("b", "a"));
    /// graph.add(BranchInfo::new("c", "b"));
    ///
    /// let stack = graph.stack("b");
    /// assert_eq!(stack, vec!["a", "b", "c"]);
    /// ```
    pub fn stack<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let mut stack = self.ancestors(name);
        stack.reverse(); // Ancestors are bottom-up, we want top-down
        stack.push(name);
        stack.extend(self.descendants(name));
        stack
    }

    /// Returns the depth of a branch (distance from trunk).
    ///
    /// - Depth 1: Direct child of trunk
    /// - Depth 2: Grandchild of trunk
    /// - etc.
    ///
    /// Returns 0 if the branch is not tracked or has trunk as parent with no path.
    pub fn depth(&self, name: &str) -> usize {
        let mut depth = 0;
        let mut current = name;

        while let Some(parent) = self.parent(current) {
            if self.is_trunk(parent) {
                return depth + 1;
            }
            depth += 1;
            current = parent;
        }

        depth
    }

    /// Checks if setting `from` to depend on `to` would create a cycle.
    ///
    /// Returns `true` if:
    /// - `from == to` (self-reference)
    /// - `to` is an ancestor of `from`
    ///
    /// # Example
    ///
    /// ```rust
    /// use stkd_core::dag::BranchGraph;
    ///
    /// let graph = BranchGraph::new("main");
    ///
    /// // Self-reference always creates a cycle
    /// assert!(graph.would_create_cycle("a", "a"));
    ///
    /// // Different branches don't create cycles (with empty graph)
    /// assert!(!graph.would_create_cycle("a", "b"));
    /// ```
    pub fn would_create_cycle(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }

        // Check if 'to' is an ancestor of 'from'
        let mut current = from;
        while let Some(parent) = self.parent(current) {
            if parent == to {
                return true;
            }
            if self.is_trunk(parent) {
                break;
            }
            current = parent;
        }

        false
    }

    /// Returns branches in topological order (parents before children).
    ///
    /// This ordering ensures that when processing branches sequentially,
    /// a branch's parent is always processed before the branch itself.
    ///
    /// Useful for operations like bulk rebasing where parents must be
    /// rebased before their children.
    pub fn topological_order(&self) -> Vec<&str> {
        let mut result = vec![];
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        fn visit<'a>(
            graph: &'a BranchGraph,
            name: &'a str,
            visited: &mut HashSet<&'a str>,
            visiting: &mut HashSet<&'a str>,
            result: &mut Vec<&'a str>,
        ) {
            if visited.contains(name) {
                return;
            }
            if visiting.contains(name) {
                // Cycle detected, skip
                return;
            }

            visiting.insert(name);

            for child in graph.children(name) {
                visit(graph, child, visited, visiting, result);
            }

            visiting.remove(name);
            visited.insert(name);
            result.push(name);
        }

        // Start from all root branches (those with trunk as parent)
        let roots: Vec<_> = self
            .branches
            .values()
            .filter(|b| self.is_trunk(&b.parent))
            .map(|b| b.name.as_str())
            .collect();

        for root in roots {
            visit(self, root, &mut visited, &mut visiting, &mut result);
        }

        result.reverse();
        result
    }

    /// Validates graph integrity.
    ///
    /// Checks for:
    /// - **Orphan branches**: Branches whose parent is neither trunk nor a tracked branch
    /// - **Cycles**: Branches that form a circular dependency
    ///
    /// # Errors
    ///
    /// Returns an error describing the first integrity issue found.
    pub fn validate(&self) -> Result<()> {
        // Check for orphan branches (parent not trunk and not tracked)
        for branch in self.branches.values() {
            if !self.is_trunk(&branch.parent) && !self.branches.contains_key(&branch.parent) {
                return Err(Error::other(format!(
                    "Branch '{}' has orphan parent '{}'",
                    branch.name, branch.parent
                )));
            }
        }

        // Check for cycles
        for name in self.branches.keys() {
            let mut visited = HashSet::new();
            let mut current = name.as_str();

            while let Some(parent) = self.parent(current) {
                if self.is_trunk(parent) {
                    break;
                }
                if !visited.insert(parent) {
                    return Err(Error::CycleDetected(format!(
                        "Cycle involving branch '{}'",
                        name
                    )));
                }
                current = parent;
            }
        }

        Ok(())
    }

    /// Finds branches that need restacking (their parent has moved).
    ///
    /// A branch needs restacking when its recorded `base_commit` no longer
    /// matches the current HEAD of its parent branch. This happens when:
    /// - The parent branch was rebased
    /// - New commits were added to the parent
    /// - The parent was merged and updated
    ///
    /// # Arguments
    ///
    /// * `repo` - The git repository to check branch positions
    ///
    /// # Errors
    ///
    /// Returns errors if git operations fail.
    pub fn needs_restack(&self, repo: &git2::Repository) -> Result<Vec<String>> {
        let mut result = vec![];

        for branch in self.branches.values() {
            if self.is_trunk(&branch.parent) {
                continue;
            }

            // Get current parent HEAD
            if let Ok(parent_ref) = repo.find_branch(&branch.parent, git2::BranchType::Local) {
                if let Some(parent_commit) = parent_ref.get().target() {
                    // Check if branch's base_commit matches parent's HEAD
                    if let Some(ref base) = branch.base_commit {
                        if base != &parent_commit.to_string() {
                            result.push(branch.name.clone());
                        }
                    }
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_branch(name: &str, parent: &str) -> BranchInfo {
        BranchInfo::new(name, parent)
    }

    #[test]
    fn test_graph_basic() {
        let mut graph = BranchGraph::new("main");

        graph.add(make_branch("feature/a", "main"));
        graph.add(make_branch("feature/b", "feature/a"));

        assert!(graph.contains("feature/a"));
        assert!(graph.contains("feature/b"));
        assert!(!graph.contains("feature/c"));
    }

    #[test]
    fn test_parent_children() {
        let mut graph = BranchGraph::new("main");

        graph.add(make_branch("feature/a", "main"));
        graph.add(make_branch("feature/b", "feature/a"));
        graph.add(make_branch("feature/c", "feature/a"));

        assert_eq!(graph.parent("feature/b"), Some("feature/a"));
        assert_eq!(graph.children("feature/a").len(), 2);
        assert!(graph.children("feature/a").contains(&"feature/b"));
        assert!(graph.children("feature/a").contains(&"feature/c"));
    }

    #[test]
    fn test_descendants() {
        let mut graph = BranchGraph::new("main");

        graph.add(make_branch("a", "main"));
        graph.add(make_branch("b", "a"));
        graph.add(make_branch("c", "b"));

        let desc = graph.descendants("a");
        assert_eq!(desc.len(), 2);
        assert!(desc.contains(&"b"));
        assert!(desc.contains(&"c"));
    }

    #[test]
    fn test_ancestors() {
        let mut graph = BranchGraph::new("main");

        graph.add(make_branch("a", "main"));
        graph.add(make_branch("b", "a"));
        graph.add(make_branch("c", "b"));

        let anc = graph.ancestors("c");
        assert_eq!(anc.len(), 2);
        assert!(anc.contains(&"b"));
        assert!(anc.contains(&"a"));
    }

    #[test]
    fn test_depth() {
        let mut graph = BranchGraph::new("main");

        graph.add(make_branch("a", "main"));
        graph.add(make_branch("b", "a"));
        graph.add(make_branch("c", "b"));

        assert_eq!(graph.depth("a"), 1);
        assert_eq!(graph.depth("b"), 2);
        assert_eq!(graph.depth("c"), 3);
    }

    #[test]
    fn test_cycle_detection() {
        let graph = BranchGraph::new("main");
        assert!(!graph.would_create_cycle("a", "b"));
        assert!(graph.would_create_cycle("a", "a"));
    }

    #[test]
    fn test_topological_order() {
        let mut graph = BranchGraph::new("main");

        graph.add(make_branch("a", "main"));
        graph.add(make_branch("b", "a"));
        graph.add(make_branch("c", "a"));
        graph.add(make_branch("d", "b"));

        let order = graph.topological_order();

        // 'a' should come before 'b', 'c', and 'd'
        let pos_a = order.iter().position(|&x| x == "a").unwrap();
        let pos_b = order.iter().position(|&x| x == "b").unwrap();
        let pos_d = order.iter().position(|&x| x == "d").unwrap();

        assert!(pos_a < pos_b);
        assert!(pos_b < pos_d);
    }

    #[test]
    fn test_stack() {
        let mut graph = BranchGraph::new("main");

        graph.add(make_branch("a", "main"));
        graph.add(make_branch("b", "a"));
        graph.add(make_branch("c", "b"));

        let stack = graph.stack("b");
        assert_eq!(stack, vec!["a", "b", "c"]);
    }
}
