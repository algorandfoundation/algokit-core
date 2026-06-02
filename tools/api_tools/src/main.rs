use clap::{Parser, Subcommand};
use color_eyre::eyre::{Result, eyre};
use duct::cmd;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

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
    /// Format generated indexer Rust code
    #[command(name = "format-indexer")]
    FormatIndexer,
    /// Format generated KMD Rust code
    #[command(name = "format-kmd")]
    FormatKmd,
    /// Generate algod API client
    #[command(name = "generate-algod")]
    GenerateAlgod,
    /// Generate indexer API client
    #[command(name = "generate-indexer")]
    GenerateIndexer,
    /// Generate KMD API client
    #[command(name = "generate-kmd")]
    GenerateKmd,
    /// Generate both algod and indexer API clients
    #[command(name = "generate-all")]
    GenerateAll,
    /// Fetch all OAS3 specs (algod, indexer, kmd) from the pinned upstream commit
    #[command(name = "convert-openapi")]
    ConvertOpenapi,
    /// Fetch the algod OAS3 spec from the pinned upstream commit
    #[command(name = "convert-algod")]
    ConvertAlgod,
    /// Fetch the indexer OAS3 spec from the pinned upstream commit
    #[command(name = "convert-indexer")]
    ConvertIndexer,
    /// Fetch the kmd OAS3 spec from the pinned upstream commit
    #[command(name = "convert-kmd")]
    ConvertKmd,
}

fn repo_root() -> &'static Path {
    static ROOT: OnceCell<PathBuf> = OnceCell::new();

    ROOT.get_or_init(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|dir| dir.parent())
            .map(Path::to_path_buf)
            .expect("invalid repository layout")
    })
    .as_path()
}

fn run(command_str: &str, dir: Option<&Path>, env_vars: Option<&[(&str, &str)]>) -> Result<()> {
    let mut tokens = shlex::Shlex::new(command_str);
    let program = tokens
        .next()
        .ok_or_else(|| eyre!("command string must not be empty"))?;
    let args: Vec<_> = tokens.collect();

    let working_dir = dir
        .map(|path| repo_root().join(path))
        .unwrap_or_else(|| repo_root().to_path_buf());

    let mut expr = cmd(program, args).dir(&working_dir).stderr_to_stdout();

    if let Some(vars) = env_vars {
        expr = vars
            .iter()
            .fold(expr, |cmd, (key, value)| cmd.env(key, value));
    }

    expr.run()?;
    Ok(())
}

#[derive(Clone, Copy)]
struct RsClientConfig {
    spec: &'static str,
    output_rel: &'static str,
    package_name: &'static str,
    description: &'static str,
}

const ALGOD_RS_CLIENT: RsClientConfig = RsClientConfig {
    spec: "algod",
    output_rel: "crates/algod_client",
    package_name: "algod_client",
    description: "API client for algod interaction.",
};

const INDEXER_RS_CLIENT: RsClientConfig = RsClientConfig {
    spec: "indexer",
    output_rel: "crates/indexer_client",
    package_name: "indexer_client",
    description: "API client for indexer interaction.",
};

const KMD_RS_CLIENT: RsClientConfig = RsClientConfig {
    spec: "kmd",
    output_rel: "crates/kmd_client",
    package_name: "kmd_client",
    description: "API client for kmd interaction.",
};

fn generate_rs_client(config: &RsClientConfig) -> Result<()> {
    run(
        &format!(
            "uv run python -m rust_oas_generator.cli ../specs/{}.oas3.json --output ../../{}/ --package-name {} --description \"{}\"",
            config.spec, config.output_rel, config.package_name, config.description
        ),
        Some(Path::new("api/oas_generator")),
        None,
    )?;

    run(
        &format!(
            "cargo fmt --manifest-path Cargo.toml -p {}",
            config.package_name
        ),
        None,
        None,
    )?;

    Ok(())
}

/// Pinned `algokit-oas-generator` commit we consume specs from. Bump in sync with
/// `api/specs/.oas-generator-sha`, then re-run `convert-openapi`.
const OAS_GENERATOR_SHA: &str = "60ac9cc6fcb33a5fd375f724cc6f54146072eb56";

/// Fetch one OAS3 spec from the pinned upstream commit into `api/specs/`.
fn fetch_spec(spec: &str) -> Result<()> {
    let url = format!(
        "https://raw.githubusercontent.com/algorandfoundation/algokit-oas-generator/{OAS_GENERATOR_SHA}/specs/{spec}.oas3.json"
    );

    run(
        &format!("curl -fsSL {url} -o api/specs/{spec}.oas3.json"),
        None,
        None,
    )
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
        Commands::FormatIndexer => {
            run(
                "cargo fmt --manifest-path Cargo.toml -p indexer_client",
                None,
                None,
            )?;
        }
        Commands::FormatKmd => {
            run(
                "cargo fmt --manifest-path Cargo.toml -p kmd_client",
                None,
                None,
            )?;
        }
        Commands::GenerateAlgod => {
            generate_rs_client(&ALGOD_RS_CLIENT)?;
        }
        Commands::GenerateIndexer => {
            generate_rs_client(&INDEXER_RS_CLIENT)?;
        }
        Commands::GenerateKmd => {
            generate_rs_client(&KMD_RS_CLIENT)?;
        }
        Commands::GenerateAll => {
            generate_rs_client(&ALGOD_RS_CLIENT)?;
            generate_rs_client(&INDEXER_RS_CLIENT)?;
            generate_rs_client(&KMD_RS_CLIENT)?;
        }
        Commands::ConvertOpenapi => {
            fetch_spec("algod")?;
            fetch_spec("indexer")?;
            fetch_spec("kmd")?;
        }
        Commands::ConvertAlgod => {
            fetch_spec("algod")?;
        }
        Commands::ConvertIndexer => {
            fetch_spec("indexer")?;
        }
        Commands::ConvertKmd => {
            fetch_spec("kmd")?;
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
