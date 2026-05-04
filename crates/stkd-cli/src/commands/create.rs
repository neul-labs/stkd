//! Create a new branch

use anyhow::Result;
use clap::Args;
use serde::Serialize;
use stkd_core::{Repository, TemplateStore};

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

#[derive(Serialize)]
struct CreatedBranchJson {
    name: String,
    parent: String,
}

pub async fn execute(args: CreateArgs, json: bool) -> Result<()> {
    let repo = Repository::open(".")?;
    let stack_dir = repo.git().path().join("stkd");

    // List templates mode
    if args.list_templates {
        let store = TemplateStore::load(&stack_dir);
        let templates = store.list();

        if json {
            let names: Vec<String> = templates.into_iter().map(|t| t.name).collect();
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({ "templates": names }))?);
        } else {
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
        }
        return Ok(());
    }

    repo.ensure_clean()?;

    if args.from_trunk {
        repo.checkout(repo.trunk())?;
    }

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

        let mut created = Vec::new();
        for (i, branch_name) in branch_names.iter().enumerate() {
            let info = repo.create_branch(branch_name)?;
            created.push(CreatedBranchJson {
                name: info.name.clone(),
                parent: info.parent.clone(),
            });
            if !json {
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
            }
        }

        if json {
            println!("{}", serde_json::to_string_pretty(
                &serde_json::json!({ "template": template_name, "created": created })
            )?);
        } else {
            output::success(&format!(
                "Created {} branches from template '{}'",
                created.len(),
                template_name
            ));
        }

        return Ok(());
    }

    let info = repo.create_branch(&args.name)?;

    if json {
        println!("{}", serde_json::to_string_pretty(
            &serde_json::json!({ "branch": info.name, "parent": info.parent })
        )?);
    } else {
        output::success(&format!(
            "Created branch '{}' on top of '{}'",
            output::branch(&info.name, true),
            info.parent
        ));
    }

    Ok(())
}
