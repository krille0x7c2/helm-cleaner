use anyhow::{Context, Result};
use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{generate, shells::Bash};
use dialoguer::{theme::ColorfulTheme, Select};
use kube::{api::ListParams, Api, Client, ResourceExt};
use k8s_openapi::api::core::v1::{Namespace, Secret};
use std::collections::BTreeSet;
use std::io;
use std::process::Command as StdCommand;

/// Helm Cleaner CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Uninstall Helm releases
    Uninstall {
        /// Kubernetes namespace
        #[arg(short, long)]
        namespace: String,

        /// Helm release name (optional)
        #[arg(short, long)]
        release: Option<String>,

        /// Delete namespace after uninstall
        #[arg(long)]
        delete_namespace: bool,

        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,
    },

    /// Generate bash completions
    Completions,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Completions => {
            // Generate bash completion script
            generate(Bash, &mut Args::command(), "helm-cleaner", &mut io::stdout());
            return Ok(());
        }
        Commands::Uninstall {
            namespace,
            release,
            delete_namespace,
            force,
        } => {
            let client = Client::try_default().await?;

            // List Helm releases
            let releases = list_releases(&client, &namespace).await?;
            if releases.is_empty() {
                println!("No Helm releases found in namespace '{}'", namespace);
                return Ok(());
            }

            // Determine selected releases
            let selected_releases: Vec<String> = match release {
                Some(r) => vec![r],
                None => {
                    let mut selections: Vec<String> = releases.clone();
                    selections.push("<ALL RELEASES>".to_string());

                    let idx = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select a release to uninstall")
                        .items(&selections)
                        .default(0)
                        .interact()?;

                    let selection = &selections[idx];
                    if selection == "<ALL RELEASES>" {
                        releases
                    } else {
                        vec![selection.clone()]
                    }
                }
            };

            // Confirmation prompt
            if !force {
                if selected_releases.len() == 1 {
                    println!(
                        "About to uninstall release '{}' in namespace '{}'.",
                        selected_releases[0], namespace
                    );
                } else {
                    println!(
                        "About to uninstall all releases ({}) in namespace '{}'.",
                        selected_releases.join(", "),
                        namespace
                    );
                }

                if delete_namespace {
                    println!("⚠️  Namespace '{}' will also be deleted.", namespace);
                }

                println!("Proceed? [y/N]");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim().to_lowercase() != "y" {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            // Uninstall releases
            for release in &selected_releases {
                helm_uninstall(release, &namespace)?;
            }

            // Delete namespace if requested
            if delete_namespace {
                delete_ns(&client, &namespace).await?;
            }
        }
    }

    Ok(())
}

/// List Helm releases in a namespace (sorted and deduplicated)
async fn list_releases(client: &Client, ns: &str) -> Result<Vec<String>> {
    let api: Api<Secret> = Api::namespaced(client.clone(), ns);
    let lp = ListParams::default().labels("owner=helm");
    let secrets = api.list(&lp).await?;

    let mut releases_set = BTreeSet::new();
    for s in secrets {
        if let Some(name) = s.labels().get("name") {
            releases_set.insert(name.clone());
        }
    }

    Ok(releases_set.into_iter().collect())
}

/// Use Helm CLI to uninstall a release
fn helm_uninstall(release: &str, ns: &str) -> Result<()> {
    println!("Running: helm uninstall {} -n {}", release, ns);
    let status = StdCommand::new("helm")
        .args(["uninstall", release, "-n", ns])
        .status()
        .context("Failed to run helm uninstall")?;

    if !status.success() {
        anyhow::bail!("helm uninstall failed for release '{}'", release);
    }

    println!("✅ Release '{}' uninstalled from '{}'", release, ns);
    Ok(())
}

/// Delete Kubernetes namespace
async fn delete_ns(client: &Client, ns: &str) -> Result<()> {
    let ns_api: Api<Namespace> = Api::all(client.clone());
    ns_api.delete(ns, &Default::default()).await?;
    println!("✅ Namespace '{}' deleted", ns);
    Ok(())
}
