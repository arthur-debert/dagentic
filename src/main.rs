mod commands;
mod config;
mod context;
mod fs;
mod gh;
mod git;
mod labels;
mod templates;

use clap::{Parser, Subcommand};
use clapfig::{Boundary, Clapfig, SearchPath};
use config::DagenticConfig;
use context::Context;
use fs::RealFs;
use gh::GhCli;
use git::GitCli;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "gh-dagentic",
    about = "Agentic development workflow orchestration",
    version = VERSION,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up Dagentic in the current repository
    Init,
    /// Re-sync workflow files, issue templates, and labels
    Update,
    /// Show the current state of the pipeline
    Status,
}

fn main() {
    let cli = Cli::parse();
    let config: DagenticConfig = Clapfig::builder()
        .app_name("dagentic")
        .search_paths(vec![SearchPath::Ancestors(Boundary::Marker(".git"))])
        .load()
        .unwrap_or_else(|e| {
            eprintln!("Warning: could not load config: {e}");
            DagenticConfig::default()
        });
    let fs = RealFs;
    let host = GhCli;
    let repo = GitCli;
    let ctx = Context {
        config: &config,
        fs: &fs,
        host: &host,
        repo: &repo,
    };

    let result = match cli.command {
        Some(Commands::Init) => commands::init::run(&ctx),
        Some(Commands::Update) => commands::update::run(&ctx),
        Some(Commands::Status) => commands::status::run(&ctx),
        None => {
            Cli::parse_from(["gh-dagentic", "--help"]);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
