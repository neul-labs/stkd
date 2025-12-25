//! Output formatting for the CLI

use colored::Colorize;
use std::sync::atomic::{AtomicBool, Ordering};

static QUIET: AtomicBool = AtomicBool::new(false);

/// Set quiet mode
pub fn set_quiet(quiet: bool) {
    QUIET.store(quiet, Ordering::SeqCst);
}

/// Check if quiet mode is enabled
pub fn is_quiet() -> bool {
    QUIET.load(Ordering::SeqCst)
}

/// Print a success message
pub fn success(msg: &str) {
    if !is_quiet() {
        println!("{} {}", "✓".green(), msg);
    }
}

/// Print an info message
pub fn info(msg: &str) {
    if !is_quiet() {
        println!("{}", msg);
    }
}

/// Print a warning message
pub fn warn(msg: &str) {
    eprintln!("{} {}", "⚠".yellow(), msg);
}

/// Print an error message
pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red(), msg);
}

/// Print a hint
pub fn hint(msg: &str) {
    if !is_quiet() {
        println!("{} {}", "→".cyan(), msg.dimmed());
    }
}

/// Format a branch name
pub fn branch(name: &str, is_current: bool) -> String {
    if is_current {
        format!("{}", name.green().bold())
    } else {
        name.to_string()
    }
}

/// Format a PR number
pub fn pr_number(num: u64) -> String {
    format!("{}", format!("#{}", num).cyan())
}

/// Format a stack with tree structure
pub fn format_stack_tree(
    entries: &[(String, bool, usize, Option<u64>)], // (name, is_current, depth, pr_number)
) -> String {
    let mut output = String::new();

    for (i, (name, is_current, depth, pr_num)) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1
            || entries
                .get(i + 1)
                .map(|(_, _, d, _)| *d <= *depth)
                .unwrap_or(true);

        // Build the tree prefix
        let mut prefix = String::new();
        for _ in 0..*depth {
            prefix.push_str("│ ");
        }

        let connector = if is_last { "└─" } else { "├─" };
        let marker = if *is_current { "◉" } else { "○" };

        let name_str = if *is_current {
            name.green().bold().to_string()
        } else {
            name.to_string()
        };

        let pr_str = pr_num
            .map(|n| format!(" {}", format!("#{}", n).cyan()))
            .unwrap_or_default();

        output.push_str(&format!(
            "{}{} {} {}{}\n",
            prefix, connector, marker, name_str, pr_str
        ));
    }

    output
}

/// Prompt for confirmation
pub fn confirm(msg: &str) -> bool {
    use dialoguer::Confirm;

    Confirm::new()
        .with_prompt(msg)
        .default(false)
        .interact()
        .unwrap_or(false)
}

/// Select from options
pub fn select(msg: &str, options: &[&str]) -> Option<usize> {
    use dialoguer::Select;

    Select::new()
        .with_prompt(msg)
        .items(options)
        .interact_opt()
        .ok()
        .flatten()
}

/// Input a string
pub fn input(msg: &str) -> Option<String> {
    use dialoguer::Input;

    Input::new()
        .with_prompt(msg)
        .interact_text()
        .ok()
}
