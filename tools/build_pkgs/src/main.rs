mod python;
mod swift;
mod typescript;

use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::Output;

use clap::{Parser, ValueEnum, command};
use color_eyre::eyre::Result;
use duct::cmd;

#[derive(Clone, Debug, ValueEnum)]
enum Language {
    #[value(alias = "py")]
    Python,
    #[value(alias = "ts")]
    Typescript,
    Swift,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Python => f.write_str("python"),
            Language::Typescript => f.write_str("typescript"),
            Language::Swift => f.write_str("swift"),
        }
    }
}

impl Language {
    fn build(&self, pkg: &Package) -> Result<()> {
        match self {
            Self::Python => python::build(pkg),
            Self::Typescript => typescript::build(pkg),
            Self::Swift => swift::build(pkg),
        }
    }

    fn iter() -> impl Iterator<Item = Language> {
        [Self::Python, Self::Typescript, Self::Swift].into_iter()
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum Package {
    #[value(alias = "algokit_transact")]
    Transact,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Package::Transact => f.write_str("algokit_transact"),
        }
    }
}

impl Package {
    fn crate_name(&self) -> String {
        match self {
            Self::Transact => "algokit_transact_ffi",
        }
        .to_string()
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    package: Package,
    language: Option<Language>,
}

fn get_repo_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = Path::new(manifest_dir)
        .parent() // crates/
        .unwrap()
        .parent() // repo root
        .unwrap();
    repo_root.to_str().unwrap();

    PathBuf::from(repo_root)
}

fn run(
    command_str: &str,
    dir: Option<&Path>,
    env_vars: Option<HashMap<String, String>>,
) -> Result<Output> {
    let parsed_command: Vec<String> = shlex::Shlex::new(command_str).collect();

    let dir = get_repo_root().join(dir.unwrap_or(Path::new("")));
    let mut command = cmd(&parsed_command[0], &parsed_command[1..])
        .dir(&dir)
        .stderr_to_stdout();

    if let Some(env_vars) = env_vars {
        for (key, value) in &env_vars {
            command = command.env(key, value);
        }
    }

    println!("\nRunning `{command_str}` in `{}`\n", dir.display());

    Ok(command.run()?)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_BACKTRACE").is_err() {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "full");
        }
    }

    let parsed = Args::parse();
    if let Some(lang) = parsed.language {
        lang.build(&parsed.package)?;
    } else {
        Language::iter().for_each(|lang| {
            if let Err(e) = lang.build(&parsed.package) {
                eprintln!("Error building {}: {}", lang, e);
            }
        });
    }

    Ok(())
}
