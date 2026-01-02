//! Create a new branch

use anyhow::Result;
use clap::Args;
use stack_core::{Repository, TemplateStore};

use crate::output;

#[derive(Args)]
pub struct CreateArgs {
    /// Name for the new branch (or base name when using --template)
    name: String,

    /// Create branch from trunk instead of current branch
    #[arg(long)]
    from_trunk: bool,

    /// Use a template to create multiple branches
    #[arg(long, short)]
    template: Option<String>,

    /// List available templates
    #[arg(long)]
    list_templates: bool,
}

pub async fn execute(args: CreateArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let stack_dir = repo.git().path().join("stack");

    // List templates mode
    if args.list_templates {
        let store = TemplateStore::load(&stack_dir);
        let templates = store.list();

        if templates.is_empty() {
            output::info("No templates available");
        } else {
            output::info("Available templates:");
            output::info("");
            for template in templates {
                let branch_count = template.branches.len();
                let plural = if branch_count == 1 { "" } else { "es" };
                output::info(&format!(
                    "  {} - {} ({} branch{})",
                    output::bold(&template.name),
                    template.description,
                    branch_count,
                    plural
                ));
            }
            output::info("");
            output::hint("Use --template <name> to create from a template");
        }
        return Ok(());
    }

    // Ensure clean working directory
    repo.ensure_clean()?;

    // If from_trunk, checkout trunk first
    if args.from_trunk {
        repo.checkout(repo.trunk())?;
    }

    // Template mode: create multiple branches
    if let Some(template_name) = &args.template {
        let store = TemplateStore::load(&stack_dir);
        let template = store.find(template_name).ok_or_else(|| {
            anyhow::anyhow!(
                "Template '{}' not found. Use --list-templates to see available templates.",
                template_name
            )
        })?;

        let branch_names = template.generate_names(&args.name);

        if branch_names.is_empty() {
            return Err(anyhow::anyhow!("Template '{}' has no branches defined", template_name));
        }

        output::info(&format!(
            "Creating stack from template '{}':",
            template_name
        ));

        let mut created = Vec::new();
        for (i, branch_name) in branch_names.iter().enumerate() {
            let info = repo.create_branch(branch_name)?;
            let desc = &template.branches[i].description;
            if desc.is_empty() {
                output::success(&format!("  {} Created '{}'", output::CHECKMARK, branch_name));
            } else {
                output::success(&format!(
                    "  {} Created '{}' ({})",
                    output::CHECKMARK,
                    branch_name,
                    desc
                ));
            }
            created.push(info);
        }

        output::info("");
        output::success(&format!(
            "Created {} branches from template '{}'",
            created.len(),
            template_name
        ));

        // Show the stack structure
        output::info("");
        output::info("Stack structure:");
        let parent = if args.from_trunk {
            repo.trunk().to_string()
        } else {
            created.first().map(|i| i.parent.clone()).unwrap_or_default()
        };
        output::info(&format!("  {}", parent));
        for (i, info) in created.iter().enumerate() {
            let prefix = if i == created.len() - 1 { "└─" } else { "├─" };
            let marker = if i == created.len() - 1 { "●" } else { "○" };
            output::info(&format!("  {} {} {}", prefix, marker, info.name));
        }

        return Ok(());
    }

    // Single branch mode
    let info = repo.create_branch(&args.name)?;

    output::success(&format!(
        "Created branch '{}' on top of '{}'",
        output::branch(&info.name, true),
        info.parent
    ));

    Ok(())
}
