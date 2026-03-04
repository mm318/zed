use anyhow::{Context, Result};
use clap::Parser;
use gh_workflow::Workflow;
use std::fs;
use std::path::{Path, PathBuf};

mod compare_perf;
mod nix_build;
mod run_bundling;
mod run_tests;
mod runners;
mod steps;
mod vars;

#[derive(Parser)]
pub struct GenerateWorkflowArgs {}

struct WorkflowFile {
    source: fn() -> Workflow,
}

impl WorkflowFile {
    fn new(f: fn() -> Workflow) -> WorkflowFile {
        WorkflowFile { source: f }
    }

    fn generate_file(&self) -> Result<()> {
        let workflow = (self.source)();
        let workflow_folder = PathBuf::from(".github/workflows");

        fs::create_dir_all(&workflow_folder).with_context(|| {
            format!("Failed to create directory: {}", workflow_folder.display())
        })?;

        let workflow_name = workflow
            .name
            .as_ref()
            .expect("Workflow must have a name at this point");
        let filename = format!(
            "{}.yml",
            workflow_name.rsplit("::").next().unwrap_or(workflow_name)
        );

        let workflow_path = workflow_folder.join(filename);

        let content = workflow
            .to_string()
            .map_err(|e| anyhow::anyhow!("{:?}: {:?}", workflow_path, e))?;

        let disclaimer = format!(
            concat!(
                "# Generated from xtask::workflows::{}\n",
                "# Rebuild with `cargo xtask workflows`.",
            ),
            workflow_name,
        );

        let content = [disclaimer, content].join("\n");
        fs::write(&workflow_path, content).map_err(Into::into)
    }
}

pub fn run_workflows(_: GenerateWorkflowArgs) -> Result<()> {
    if !Path::new("crates/zed/").is_dir() {
        anyhow::bail!("xtask workflows must be ran from the project root");
    }

    let workflows = [
        WorkflowFile::new(compare_perf::compare_perf),
        WorkflowFile::new(run_bundling::run_bundling),
        WorkflowFile::new(run_tests::run_tests),
    ];

    for workflow_file in workflows {
        workflow_file.generate_file()?;
    }

    Ok(())
}
