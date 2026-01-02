//! Stack templates for creating predefined branch structures.
//!
//! Templates allow users to quickly create stacks with consistent naming
//! patterns and structures. For example, a "feature-with-tests" template
//! might create both a feature branch and a corresponding tests branch.
//!
//! # Built-in Templates
//!
//! - `feature`: Single feature branch
//! - `feature-tests`: Feature branch with accompanying tests branch
//! - `refactor`: Three-branch refactor stack (prep, main, cleanup)
//!
//! # Custom Templates
//!
//! Custom templates can be defined in `.git/stack/templates.json`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A branch definition within a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateBranch {
    /// Suffix to append to the base name (e.g., "-tests" creates "feature-tests")
    /// Empty string means use base name directly
    pub suffix: String,

    /// Description of this branch's purpose
    #[serde(default)]
    pub description: String,
}

/// A stack template defining a series of branches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackTemplate {
    /// Template name (e.g., "feature-tests")
    pub name: String,

    /// Description of what this template is for
    pub description: String,

    /// Branch definitions in order (first is created first, becomes parent of second, etc.)
    pub branches: Vec<TemplateBranch>,
}

impl StackTemplate {
    /// Create a new template
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            branches: Vec::new(),
        }
    }

    /// Add a branch to the template
    pub fn with_branch(mut self, suffix: impl Into<String>, description: impl Into<String>) -> Self {
        self.branches.push(TemplateBranch {
            suffix: suffix.into(),
            description: description.into(),
        });
        self
    }

    /// Generate branch names from a base name
    pub fn generate_names(&self, base_name: &str) -> Vec<String> {
        self.branches
            .iter()
            .map(|b| {
                if b.suffix.is_empty() {
                    base_name.to_string()
                } else {
                    format!("{}{}", base_name, b.suffix)
                }
            })
            .collect()
    }

    /// Get built-in templates
    pub fn builtins() -> Vec<StackTemplate> {
        vec![
            StackTemplate::new("feature", "Single feature branch")
                .with_branch("", "Main feature implementation"),

            StackTemplate::new("feature-tests", "Feature with tests branch")
                .with_branch("", "Main feature implementation")
                .with_branch("-tests", "Tests for the feature"),

            StackTemplate::new("refactor", "Three-stage refactor stack")
                .with_branch("-prep", "Preparatory refactoring")
                .with_branch("", "Main refactoring changes")
                .with_branch("-cleanup", "Post-refactor cleanup"),

            StackTemplate::new("migration", "Database migration stack")
                .with_branch("-schema", "Schema changes")
                .with_branch("-data", "Data migration")
                .with_branch("-cleanup", "Remove old code"),
        ]
    }

    /// Find a built-in template by name
    pub fn find_builtin(name: &str) -> Option<StackTemplate> {
        Self::builtins().into_iter().find(|t| t.name == name)
    }
}

/// Template storage and management
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateStore {
    /// User-defined templates
    pub templates: HashMap<String, StackTemplate>,
}

impl TemplateStore {
    /// Load templates from disk
    pub fn load(stack_dir: &Path) -> Self {
        let path = stack_dir.join("templates.json");
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(store) = serde_json::from_str(&content) {
                    return store;
                }
            }
        }
        Self::default()
    }

    /// Save templates to disk
    pub fn save(&self, stack_dir: &Path) -> std::io::Result<()> {
        let path = stack_dir.join("templates.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }

    /// Find a template by name (checks user templates first, then builtins)
    pub fn find(&self, name: &str) -> Option<StackTemplate> {
        self.templates
            .get(name)
            .cloned()
            .or_else(|| StackTemplate::find_builtin(name))
    }

    /// Add a user-defined template
    pub fn add(&mut self, template: StackTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// Remove a user-defined template
    pub fn remove(&mut self, name: &str) -> Option<StackTemplate> {
        self.templates.remove(name)
    }

    /// List all available templates (user + builtin)
    pub fn list(&self) -> Vec<StackTemplate> {
        let mut templates: Vec<_> = self.templates.values().cloned().collect();

        // Add builtins that aren't overridden
        for builtin in StackTemplate::builtins() {
            if !self.templates.contains_key(&builtin.name) {
                templates.push(builtin);
            }
        }

        templates.sort_by(|a, b| a.name.cmp(&b.name));
        templates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_names() {
        let template = StackTemplate::new("test", "Test template")
            .with_branch("", "Main")
            .with_branch("-tests", "Tests");

        let names = template.generate_names("my-feature");
        assert_eq!(names, vec!["my-feature", "my-feature-tests"]);
    }

    #[test]
    fn test_builtins() {
        let builtins = StackTemplate::builtins();
        assert!(builtins.len() >= 3);

        let feature = StackTemplate::find_builtin("feature").unwrap();
        assert_eq!(feature.branches.len(), 1);

        let refactor = StackTemplate::find_builtin("refactor").unwrap();
        assert_eq!(refactor.branches.len(), 3);
    }

    #[test]
    fn test_template_store() {
        let mut store = TemplateStore::default();

        let custom = StackTemplate::new("custom", "Custom template")
            .with_branch("", "Main")
            .with_branch("-review", "Review changes");

        store.add(custom);

        assert!(store.find("custom").is_some());
        assert!(store.find("feature").is_some()); // builtin
        assert!(store.find("nonexistent").is_none());
    }
}
