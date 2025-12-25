//! Directed Acyclic Graph for branch dependencies
//!
//! Manages the dependency relationships between branches in a stack.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::branch::BranchInfo;
use crate::{Error, Result};

/// Branch dependency graph
#[derive(Debug, Default)]
pub struct BranchGraph {
    /// Map of branch name -> branch info
    branches: HashMap<String, BranchInfo>,
    /// Name of the trunk branch
    trunk: String,
}

impl BranchGraph {
    /// Create a new graph with the given trunk
    pub fn new(trunk: impl Into<String>) -> Self {
        Self {
            branches: HashMap::new(),
            trunk: trunk.into(),
        }
    }

    /// Load graph from a list of branches
    pub fn from_branches(branches: Vec<BranchInfo>, trunk: impl Into<String>) -> Self {
        let mut graph = Self::new(trunk);
        for branch in branches {
            graph.branches.insert(branch.name.clone(), branch);
        }
        graph
    }

    /// Get the trunk name
    pub fn trunk(&self) -> &str {
        &self.trunk
    }

    /// Check if a branch name is the trunk
    pub fn is_trunk(&self, name: &str) -> bool {
        name == self.trunk
    }

    /// Add a branch to the graph
    pub fn add(&mut self, info: BranchInfo) {
        self.branches.insert(info.name.clone(), info);
    }

    /// Remove a branch from the graph
    pub fn remove(&mut self, name: &str) -> Option<BranchInfo> {
        self.branches.remove(name)
    }

    /// Get a branch by name
    pub fn get(&self, name: &str) -> Option<&BranchInfo> {
        self.branches.get(name)
    }

    /// Get a mutable branch by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut BranchInfo> {
        self.branches.get_mut(name)
    }

    /// Check if a branch is tracked
    pub fn contains(&self, name: &str) -> bool {
        self.branches.contains_key(name)
    }

    /// Get all tracked branches
    pub fn all_branches(&self) -> impl Iterator<Item = &BranchInfo> {
        self.branches.values()
    }

    /// Get the parent of a branch
    pub fn parent(&self, name: &str) -> Option<&str> {
        self.branches.get(name).map(|b| b.parent.as_str())
    }

    /// Get children of a branch
    pub fn children(&self, name: &str) -> Vec<&str> {
        self.branches
            .values()
            .filter(|b| b.parent == name)
            .map(|b| b.name.as_str())
            .collect()
    }

    /// Get all descendants of a branch (recursive children)
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

    /// Get ancestors of a branch (up to trunk)
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

    /// Get the full stack containing a branch (ancestors + self + descendants)
    pub fn stack<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let mut stack = self.ancestors(name);
        stack.reverse(); // Ancestors are bottom-up, we want top-down
        stack.push(name);
        stack.extend(self.descendants(name));
        stack
    }

    /// Get depth of a branch (distance from trunk)
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

    /// Check if adding a dependency would create a cycle
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

    /// Get branches in topological order (parents before children)
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

    /// Validate graph integrity
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

    /// Find branches that need rebasing (their parent has moved)
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
