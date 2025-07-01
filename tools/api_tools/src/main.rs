use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Output;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use duct::cmd;

#[derive(Parser, Debug)]
#[command(author, version, about = "API development tools", long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Test the OAS generator
    #[command(name = "test-oas")]
    TestOas,
    /// Format the OAS generator code
    #[command(name = "format-oas")]
    FormatOas,
    /// Lint and type-check the OAS generator
    #[command(name = "lint-oas")]
    LintOas,
    /// Format generated Rust code
    #[command(name = "format-algod")]
    FormatAlgod,
    /// Generate algod API client
    #[command(name = "generate-algod")]
    GenerateAlgod,
    /// Convert OpenAPI specification
    #[command(name = "convert-openapi")]
    ConvertOpenapi,
}

fn get_repo_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = Path::new(manifest_dir)
        .parent() // tools/
        .unwrap()
        .parent() // repo root
        .unwrap();

    PathBuf::from(repo_root)
}

fn run(
    command_str: &str,
    dir: Option<&Path>,
    env_vars: Option<HashMap<String, String>>,
) -> Result<Output> {
    let parsed_command: Vec<String> = shlex::Shlex::new(command_str).collect();

    let working_dir = get_repo_root().join(dir.unwrap_or(Path::new("")));
    let mut command = cmd(&parsed_command[0], &parsed_command[1..])
        .dir(&working_dir)
        .stderr_to_stdout();

    if let Some(env_vars) = env_vars {
        for (key, value) in &env_vars {
            command = command.env(key, value);
        }
    }

    Ok(command.run()?)
}

fn execute_command(command: &Commands) -> Result<()> {
    match command {
        Commands::TestOas => {
            run("uv run pytest", Some(Path::new("api/oas_generator")), None)?;
        }
        Commands::FormatOas => {
            run(
                "uv run ruff format",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
        }
        Commands::LintOas => {
            run(
                "uv run ruff check",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            run(
                "uv run mypy rust_oas_generator",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
        }
        Commands::FormatAlgod => {
            run(
                "cargo fmt --manifest-path Cargo.toml -p algod_client",
                None,
                None,
            )?;
        }
        Commands::GenerateAlgod => {
            // Generate the client
            run(
                "uv run python -m rust_oas_generator.cli ../specs/algod.oas3.json --output ../../crates/algod_client/ --package-name algod_client --description \"API client for algod interaction.\"",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            // Format the generated code
            run(
                "cargo fmt --manifest-path Cargo.toml -p algod_client",
                None,
                None,
            )?;
        }
        Commands::ConvertOpenapi => {
            run(
                "bun scripts/convert-openapi.ts",
                Some(Path::new("api")),
                None,
            )?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_BACKTRACE").is_err() {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "full");
        }
    }

    let args = Args::parse();
    execute_command(&args.command)?;

    Ok(())
}
