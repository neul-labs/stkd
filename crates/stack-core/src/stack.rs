//! Stack representation and operations
//!
//! A Stack is a view of related branches with their dependency chain.

use crate::branch::{Branch, BranchInfo, BranchStatus};
use crate::dag::BranchGraph;

/// A stack of related branches
#[derive(Debug)]
pub struct Stack<'a> {
    /// The branches in this stack, ordered from root to tip
    entries: Vec<StackEntry<'a>>,
    /// Name of the current (checked out) branch
    current: Option<String>,
    /// Name of the trunk branch
    trunk: String,
}

impl<'a> Stack<'a> {
    /// Create a stack from a graph centered on a branch
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

    /// Get the trunk name
    pub fn trunk(&self) -> &str {
        &self.trunk
    }

    /// Get the current branch name
    pub fn current(&self) -> Option<&str> {
        self.current.as_deref()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the number of branches in the stack
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get the root (first) branch
    pub fn root(&self) -> Option<&StackEntry<'a>> {
        self.entries.first()
    }

    /// Get the tip (last) branch
    pub fn tip(&self) -> Option<&StackEntry<'a>> {
        self.entries.last()
    }

    /// Get a branch by index
    pub fn get(&self, index: usize) -> Option<&StackEntry<'a>> {
        self.entries.get(index)
    }

    /// Iterate over entries
    pub fn iter(&self) -> impl Iterator<Item = &StackEntry<'a>> {
        self.entries.iter()
    }

    /// Get the index of a branch by name
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.entries.iter().position(|e| e.branch.name() == name)
    }

    /// Get the branch above the current one (toward tip)
    pub fn up(&self) -> Option<&StackEntry<'a>> {
        let current_idx = self.current.as_ref().and_then(|c| self.index_of(c))?;
        self.entries.get(current_idx + 1)
    }

    /// Get the branch below the current one (toward root)
    pub fn down(&self) -> Option<&StackEntry<'a>> {
        let current_idx = self.current.as_ref().and_then(|c| self.index_of(c))?;
        if current_idx > 0 {
            self.entries.get(current_idx - 1)
        } else {
            None
        }
    }

    /// Get branches from current to tip
    pub fn to_tip(&self) -> Vec<&StackEntry<'a>> {
        if let Some(idx) = self.current.as_ref().and_then(|c| self.index_of(c)) {
            self.entries[idx..].iter().collect()
        } else {
            vec![]
        }
    }

    /// Get branches from root to current
    pub fn to_root(&self) -> Vec<&StackEntry<'a>> {
        if let Some(idx) = self.current.as_ref().and_then(|c| self.index_of(c)) {
            self.entries[..=idx].iter().collect()
        } else {
            vec![]
        }
    }

    /// Check if all PRs in the stack are approved
    pub fn all_approved(&self) -> bool {
        self.entries.iter().all(|e| {
            e.branch
                .review_status()
                .map_or(false, |s| s == crate::branch::ReviewStatus::Approved)
        })
    }

    /// Check if all CI checks passed
    pub fn all_ci_passed(&self) -> bool {
        self.entries.iter().all(|e| {
            e.branch
                .ci_status()
                .map_or(true, |s| s == crate::branch::CiStatus::Passed)
        })
    }

    /// Get branches that need PRs
    pub fn needs_pr(&self) -> Vec<&StackEntry<'a>> {
        self.entries
            .iter()
            .filter(|e| e.branch.pr_number().is_none())
            .collect()
    }

    /// Get branches with open PRs
    pub fn has_pr(&self) -> Vec<&StackEntry<'a>> {
        self.entries
            .iter()
            .filter(|e| e.branch.pr_number().is_some())
            .collect()
    }
}

/// An entry in a stack
#[derive(Debug)]
pub struct StackEntry<'a> {
    /// The branch
    pub branch: Branch<'a>,
    /// Direct children of this branch
    pub children: Vec<String>,
}

impl<'a> StackEntry<'a> {
    /// Get the branch name
    pub fn name(&self) -> &str {
        self.branch.name()
    }

    /// Get PR number if any
    pub fn pr_number(&self) -> Option<u64> {
        self.branch.pr_number()
    }

    /// Get the status
    pub fn status(&self) -> BranchStatus {
        self.branch.status()
    }

    /// Get the depth
    pub fn depth(&self) -> usize {
        self.branch.depth()
    }

    /// Check if this is the current branch
    pub fn is_current(&self) -> bool {
        self.branch.is_current()
    }

    /// Check if this branch has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Format a stack for display
pub fn format_stack(stack: &Stack<'_>, short: bool) -> String {
    let mut output = String::new();

    for entry in stack.iter() {
        let indent = "  ".repeat(entry.depth().saturating_sub(1));
        let marker = if entry.is_current() { "◉" } else { "○" };
        let pr_info = if let Some(pr) = entry.pr_number() {
            format!(" (#{})", pr)
        } else {
            String::new()
        };

        if short {
            output.push_str(&format!("{}{} {}{}\n", indent, marker, entry.name(), pr_info));
        } else {
            let status = format!("[{}]", entry.status());
            output.push_str(&format!(
                "{}{} {} {}{}\n",
                indent,
                marker,
                entry.name(),
                status,
                pr_info
            ));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

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
