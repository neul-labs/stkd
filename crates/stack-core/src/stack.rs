//! Stack representation and operations.
//!
//! A [`Stack`] is a view of related branches organized as a linear dependency chain.
//! It provides navigation and querying capabilities for working with stacked diffs.
//!
//! # Stacked Diffs Concept
//!
//! In stacked diffs workflows, changes are organized as a series of dependent branches
//! (or merge requests/PRs), where each branch builds upon its parent:
//!
//! ```text
//! main
//!  └── feature/step-1    (depends on main)
//!       └── feature/step-2    (depends on step-1)
//!            └── feature/step-3    (depends on step-2)
//! ```
//!
//! This allows developers to:
//! - Break large changes into reviewable pieces
//! - Get early feedback on foundational changes
//! - Land changes incrementally as they're approved
//!
//! # Stack Structure
//!
//! A [`Stack`] represents this chain as an ordered list of [`StackEntry`] items,
//! from the **root** (closest to trunk) to the **tip** (furthest from trunk).
//!
//! ```text
//! Stack entries (ordered):
//!   [0] feature/step-1  ← root (depth: 1)
//!   [1] feature/step-2        (depth: 2)
//!   [2] feature/step-3  ← tip (depth: 3)
//! ```
//!
//! # Navigation
//!
//! The stack provides navigation relative to the current (checked out) branch:
//!
//! - [`Stack::up`] - move toward the tip (child branches)
//! - [`Stack::down`] - move toward the root (parent branches)
//! - [`Stack::to_tip`] - all branches from current to tip
//! - [`Stack::to_root`] - all branches from root to current
//!
//! # Example
//!
//! ```rust,no_run
//! use stack_core::dag::BranchGraph;
//! use stack_core::stack::Stack;
//!
//! // Build a graph with three stacked branches
//! let mut graph = BranchGraph::new("main");
//! // ... add branches ...
//!
//! // Create a stack view centered on a branch
//! let stack = Stack::from_graph(&graph, "feature/step-2", Some("feature/step-2"));
//!
//! // Navigate the stack
//! if let Some(parent) = stack.down() {
//!     println!("Parent branch: {}", parent.name());
//! }
//!
//! // Check merge request status
//! for entry in stack.needs_merge_request() {
//!     println!("Branch {} needs an MR", entry.name());
//! }
//! ```

use crate::branch::{Branch, BranchStatus};
use crate::dag::BranchGraph;

/// A stack of related branches forming a dependency chain.
///
/// A `Stack` represents a linear chain of branches from the trunk (e.g., `main`)
/// to the tip of development. It provides navigation and querying methods for
/// working with stacked diffs workflows.
///
/// # Structure
///
/// - **Root**: The branch closest to trunk (first entry)
/// - **Tip**: The branch furthest from trunk (last entry)
/// - **Current**: The currently checked-out branch
///
/// # Creating a Stack
///
/// Stacks are created from a [`BranchGraph`] using [`Stack::from_graph`]:
///
/// ```rust,no_run
/// use stack_core::dag::BranchGraph;
/// use stack_core::stack::Stack;
///
/// let graph = BranchGraph::new("main");
/// // ... add branches to graph ...
///
/// let stack = Stack::from_graph(&graph, "my-feature", Some("my-feature"));
/// ```
///
/// # Navigation Example
///
/// ```rust,no_run
/// # use stack_core::dag::BranchGraph;
/// # use stack_core::stack::Stack;
/// # let graph = BranchGraph::new("main");
/// # let stack = Stack::from_graph(&graph, "my-branch", Some("my-branch"));
/// // Move up (toward tip/children)
/// if let Some(child) = stack.up() {
///     println!("Next branch: {}", child.name());
/// }
///
/// // Move down (toward root/parent)
/// if let Some(parent) = stack.down() {
///     println!("Previous branch: {}", parent.name());
/// }
/// ```
#[derive(Debug)]
pub struct Stack<'a> {
    /// The branches in this stack, ordered from root to tip.
    entries: Vec<StackEntry<'a>>,
    /// Name of the current (checked out) branch.
    current: Option<String>,
    /// Name of the trunk branch (e.g., "main" or "master").
    trunk: String,
}

impl<'a> Stack<'a> {
    /// Creates a stack view from a branch graph, centered on a specific branch.
    ///
    /// This traverses the graph to find all branches in the same stack as `center`,
    /// ordering them from root (closest to trunk) to tip (furthest from trunk).
    ///
    /// # Arguments
    ///
    /// * `graph` - The branch graph to extract the stack from
    /// * `center` - The branch to center the stack view on
    /// * `current` - The currently checked-out branch name (for navigation)
    ///
    /// # Returns
    ///
    /// A new `Stack` containing all branches in the dependency chain.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use stack_core::dag::BranchGraph;
    /// use stack_core::stack::Stack;
    ///
    /// let graph = BranchGraph::new("main");
    /// // ... add branches ...
    ///
    /// // Create stack centered on "feature-2" with it as current branch
    /// let stack = Stack::from_graph(&graph, "feature-2", Some("feature-2"));
    /// ```
    pub fn from_graph(graph: &'a BranchGraph, center: &str, current: Option<&str>) -> Self {
        let mut entries = vec![];
        let trunk = graph.trunk().to_string();

        // Get all branches in the stack
        let branch_names = graph.stack(center);

        for name in branch_names {
            if let Some(info) = graph.get(name) {
                let depth = graph.depth(name);
                let is_current = current == Some(name);
                let children: Vec<_> = graph.children(name).iter().map(|s| s.to_string()).collect();

                entries.push(StackEntry {
                    branch: Branch::new(info, is_current, depth),
                    children,
                });
            }
        }

        Self {
            entries,
            current: current.map(String::from),
            trunk,
        }
    }

    /// Returns the trunk branch name (e.g., "main" or "master").
    pub fn trunk(&self) -> &str {
        &self.trunk
    }

    /// Returns the name of the currently checked-out branch, if any.
    pub fn current(&self) -> Option<&str> {
        self.current.as_deref()
    }

    /// Returns `true` if the stack contains no branches.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of branches in the stack.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns the root (first) branch in the stack.
    ///
    /// The root is the branch closest to trunk, at the base of the stack.
    pub fn root(&self) -> Option<&StackEntry<'a>> {
        self.entries.first()
    }

    /// Returns the tip (last) branch in the stack.
    ///
    /// The tip is the branch furthest from trunk, at the top of the stack.
    pub fn tip(&self) -> Option<&StackEntry<'a>> {
        self.entries.last()
    }

    /// Returns the stack entry at the given index.
    ///
    /// Index 0 is the root, and `len() - 1` is the tip.
    pub fn get(&self, index: usize) -> Option<&StackEntry<'a>> {
        self.entries.get(index)
    }

    /// Returns an iterator over all entries in the stack.
    ///
    /// Entries are ordered from root to tip.
    pub fn iter(&self) -> impl Iterator<Item = &StackEntry<'a>> {
        self.entries.iter()
    }

    /// Returns the index of a branch by name.
    ///
    /// Returns `None` if the branch is not in this stack.
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.entries.iter().position(|e| e.branch.name() == name)
    }

    /// Returns the branch above the current one (toward the tip).
    ///
    /// In stack terminology, "up" means moving toward child branches,
    /// away from trunk.
    ///
    /// # Returns
    ///
    /// - `Some(entry)` - The next branch toward the tip
    /// - `None` - If current branch is at the tip, or no current branch
    pub fn up(&self) -> Option<&StackEntry<'a>> {
        let current_idx = self.current.as_ref().and_then(|c| self.index_of(c))?;
        self.entries.get(current_idx + 1)
    }

    /// Returns the branch below the current one (toward the root).
    ///
    /// In stack terminology, "down" means moving toward parent branches,
    /// closer to trunk.
    ///
    /// # Returns
    ///
    /// - `Some(entry)` - The next branch toward the root
    /// - `None` - If current branch is at the root, or no current branch
    pub fn down(&self) -> Option<&StackEntry<'a>> {
        let current_idx = self.current.as_ref().and_then(|c| self.index_of(c))?;
        if current_idx > 0 {
            self.entries.get(current_idx - 1)
        } else {
            None
        }
    }

    /// Returns all branches from the current branch to the tip (inclusive).
    ///
    /// This is useful for operations that need to update all downstream
    /// branches (e.g., restacking after a rebase).
    pub fn to_tip(&self) -> Vec<&StackEntry<'a>> {
        if let Some(idx) = self.current.as_ref().and_then(|c| self.index_of(c)) {
            self.entries[idx..].iter().collect()
        } else {
            vec![]
        }
    }

    /// Returns all branches from the root to the current branch (inclusive).
    ///
    /// This is useful for operations that need all upstream branches
    /// (e.g., checking if dependencies are merged).
    pub fn to_root(&self) -> Vec<&StackEntry<'a>> {
        if let Some(idx) = self.current.as_ref().and_then(|c| self.index_of(c)) {
            self.entries[..=idx].iter().collect()
        } else {
            vec![]
        }
    }

    /// Returns `true` if all merge requests in the stack are approved.
    ///
    /// Returns `false` if any MR lacks approval status or is not approved.
    pub fn all_approved(&self) -> bool {
        self.entries.iter().all(|e| {
            e.branch
                .review_status()
                .map_or(false, |s| s == crate::branch::ReviewStatus::Approved)
        })
    }

    /// Returns `true` if all CI checks have passed for the stack.
    ///
    /// Returns `true` for branches without CI status (assumes passing).
    pub fn all_ci_passed(&self) -> bool {
        self.entries.iter().all(|e| {
            e.branch
                .ci_status()
                .map_or(true, |s| s == crate::branch::CiStatus::Passed)
        })
    }

    /// Returns branches that don't have merge requests yet.
    ///
    /// Useful for identifying branches that need `gt submit`.
    pub fn needs_merge_request(&self) -> Vec<&StackEntry<'a>> {
        self.entries
            .iter()
            .filter(|e| e.branch.merge_request_id().is_none())
            .collect()
    }

    /// Alias for [`needs_merge_request`](Self::needs_merge_request).
    #[deprecated(note = "Use needs_merge_request() instead")]
    #[allow(dead_code)]
    pub fn needs_pr(&self) -> Vec<&StackEntry<'a>> {
        self.needs_merge_request()
    }

    /// Returns branches that have open merge requests.
    ///
    /// Useful for syncing MR status or checking what's ready to land.
    pub fn has_merge_request(&self) -> Vec<&StackEntry<'a>> {
        self.entries
            .iter()
            .filter(|e| e.branch.merge_request_id().is_some())
            .collect()
    }

    /// Alias for [`has_merge_request`](Self::has_merge_request).
    #[deprecated(note = "Use has_merge_request() instead")]
    #[allow(dead_code)]
    pub fn has_pr(&self) -> Vec<&StackEntry<'a>> {
        self.has_merge_request()
    }
}

/// An entry in a [`Stack`], representing a single branch with its metadata.
///
/// Each entry contains:
/// - A [`Branch`] with name, status, and merge request info
/// - A list of child branch names (branches that depend on this one)
///
/// # Example
///
/// ```rust,no_run
/// # use stack_core::dag::BranchGraph;
/// # use stack_core::stack::Stack;
/// # let graph = BranchGraph::new("main");
/// # let stack = Stack::from_graph(&graph, "branch", None);
/// for entry in stack.iter() {
///     println!("Branch: {} (depth {})", entry.name(), entry.depth());
///     if let Some(mr_id) = entry.merge_request_id() {
///         println!("  MR #{}", mr_id);
///     }
///     if entry.has_children() {
///         println!("  Children: {:?}", entry.children);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct StackEntry<'a> {
    /// The branch with its metadata.
    pub branch: Branch<'a>,
    /// Names of branches that directly depend on this one.
    pub children: Vec<String>,
}

impl<'a> StackEntry<'a> {
    /// Returns the branch name.
    pub fn name(&self) -> &str {
        self.branch.name()
    }

    /// Returns the merge request ID if one has been created.
    ///
    /// Returns `None` if no MR/PR has been submitted for this branch.
    pub fn merge_request_id(&self) -> Option<u64> {
        self.branch.merge_request_id()
    }

    /// Alias for [`merge_request_id`](Self::merge_request_id).
    #[deprecated(note = "Use merge_request_id() instead")]
    #[allow(dead_code)]
    pub fn pr_number(&self) -> Option<u64> {
        self.merge_request_id()
    }

    /// Returns the branch status (synced, needs restack, etc.).
    pub fn status(&self) -> BranchStatus {
        self.branch.status()
    }

    /// Returns the depth of this branch in the stack.
    ///
    /// Depth indicates how many branches are between this one and trunk:
    /// - Depth 1: Direct child of trunk
    /// - Depth 2: Grandchild of trunk
    /// - etc.
    pub fn depth(&self) -> usize {
        self.branch.depth()
    }

    /// Returns `true` if this is the currently checked-out branch.
    pub fn is_current(&self) -> bool {
        self.branch.is_current()
    }

    /// Returns `true` if this branch has child branches depending on it.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Formats a stack for terminal display.
///
/// Produces a tree-like visualization of the stack with:
/// - Indentation based on depth
/// - `◉` marker for the current branch, `○` for others
/// - Optional status and merge request numbers
///
/// # Arguments
///
/// * `stack` - The stack to format
/// * `short` - If `true`, omit status; if `false`, include `[status]`
///
/// # Returns
///
/// A formatted string suitable for terminal output.
///
/// # Example Output
///
/// ```text
/// ○ feature/step-1 [synced] (#123)
///   ◉ feature/step-2 [needs-restack]
///     ○ feature/step-3 [synced] (#125)
/// ```
pub fn format_stack(stack: &Stack<'_>, short: bool) -> String {
    let mut output = String::new();

    for entry in stack.iter() {
        let indent = "  ".repeat(entry.depth().saturating_sub(1));
        let marker = if entry.is_current() { "◉" } else { "○" };
        let mr_info = if let Some(id) = entry.merge_request_id() {
            format!(" (#{})", id)
        } else {
            String::new()
        };

        if short {
            output.push_str(&format!("{}{} {}{}\n", indent, marker, entry.name(), mr_info));
        } else {
            let status = format!("[{}]", entry.status());
            output.push_str(&format!(
                "{}{} {} {}{}\n",
                indent,
                marker,
                entry.name(),
                status,
                mr_info
            ));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::branch::BranchInfo;

    fn make_branch(name: &str, parent: &str) -> BranchInfo {
        BranchInfo::new(name, parent)
    }

    #[test]
    fn test_stack_creation() {
        let mut graph = BranchGraph::new("main");
        graph.add(make_branch("a", "main"));
        graph.add(make_branch("b", "a"));
        graph.add(make_branch("c", "b"));

        let stack = Stack::from_graph(&graph, "b", Some("b"));

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.current(), Some("b"));
    }

    #[test]
    fn test_stack_navigation() {
        let mut graph = BranchGraph::new("main");
        graph.add(make_branch("a", "main"));
        graph.add(make_branch("b", "a"));
        graph.add(make_branch("c", "b"));

        let stack = Stack::from_graph(&graph, "b", Some("b"));

        assert!(stack.up().is_some());
        assert_eq!(stack.up().unwrap().name(), "c");

        assert!(stack.down().is_some());
        assert_eq!(stack.down().unwrap().name(), "a");
    }

    #[test]
    fn test_stack_root_tip() {
        let mut graph = BranchGraph::new("main");
        graph.add(make_branch("a", "main"));
        graph.add(make_branch("b", "a"));
        graph.add(make_branch("c", "b"));

        let stack = Stack::from_graph(&graph, "b", Some("b"));

        assert_eq!(stack.root().unwrap().name(), "a");
        assert_eq!(stack.tip().unwrap().name(), "c");
    }

    #[test]
    fn test_format_stack() {
        let mut graph = BranchGraph::new("main");
        graph.add(make_branch("feature/a", "main"));
        graph.add(make_branch("feature/b", "feature/a"));

        let stack = Stack::from_graph(&graph, "feature/a", Some("feature/a"));
        let output = format_stack(&stack, true);

        assert!(output.contains("feature/a"));
        assert!(output.contains("feature/b"));
    }
}
