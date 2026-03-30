use clap::{Parser, Subcommand};
use std::process::Command;
use std::sync::{LazyLock, Mutex};
use thiserror::Error;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn main() -> Result {
    tracing_subscriber::fmt::init();

    tracing::debug!("Parsing CLI arguments...");
    let args = CLIArgs::parse();
    tracing::debug!("Parsed CLI arguments.");

    match &args.action {
        Action::All => all()?,
        Action::Build => build()?,
        Action::Check => check()?,
        Action::Clippy => clippy()?,
        Action::Test => test()?,
        Action::Doc => doc()?,
    }

    Ok(())
}

#[derive(Parser)]
struct CLIArgs {
    #[clap(default_value_t = Action::All)]
    action: Action,
}

#[derive(Subcommand, Clone)]
enum Action {
    All,
    Build,
    Check,
    Clippy,
    Test,
    Doc,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Build => write!(f, "build"),
            Self::Check => write!(f, "check"),
            Self::Clippy => write!(f, "clippy"),
            Self::Test => write!(f, "test"),
            Self::Doc => write!(f, "doc"),
        }
    }
}

impl std::str::FromStr for Action {
    type Err = ActionParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "build" => Ok(Self::Build),
            "check" => Ok(Self::Check),
            "clippy" => Ok(Self::Clippy),
            "test" => Ok(Self::Test),
            "doc" => Ok(Self::Doc),
            v => Err(Self::Err::ParseError(v.to_string())),
        }
    }
}

#[derive(Debug, Error)]
enum ActionParseError {
    ParseError(String),
}

impl std::fmt::Display for ActionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(s) => write!(f, "ParseError by \"{}\"", s),
        }
    }
}

static CARGO: LazyLock<Mutex<String>> = LazyLock::new(|| {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    Mutex::new(cargo)
});

#[tracing::instrument]
fn all() -> Result {
    tracing::info!("Running...");
    build()?;
    check()?;
    clippy()?;
    test()?;
    doc()?;
    tracing::info!("Finished.");

    Ok(())
}

#[tracing::instrument]
fn build() -> Result {
    tracing::info!("Running...");

    // cargo build --workspace
    {
        let mut build_command = Command::new(CARGO.lock()?.as_str());
        build_command.arg("build");
        build_command.arg("--workspace");

        let exit_status = build_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo build --workspace is failed");
        }
    }

    // cargo build --release --workspace
    {
        let mut build_release_command = Command::new(CARGO.lock()?.as_str());
        build_release_command.arg("build");
        build_release_command.arg("--release");
        build_release_command.arg("--workspace");

        let exit_status = build_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo build --release --workspace is failed");
        }
    }

    tracing::info!("Finished.");
    Ok(())
}

#[tracing::instrument]
fn check() -> Result {
    tracing::info!("Running...");

    // cargo check --workspace
    {
        let mut check_command = Command::new(CARGO.lock()?.as_str());
        check_command.arg("check");
        check_command.arg("--workspace");

        let exit_status = check_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo check --workspace is failed");
        }
    }

    // cargo check --release --workspace
    {
        let mut check_release_command = Command::new(CARGO.lock()?.as_str());
        check_release_command.arg("check");
        check_release_command.arg("--release");
        check_release_command.arg("--workspace");

        let exit_status = check_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo check --release --workspace is failed");
        }
    }

    tracing::info!("Finished.");
    Ok(())
}

#[tracing::instrument]
fn clippy() -> Result {
    tracing::info!("Running...");

    // cargo clippy --workspace
    {
        let mut clippy_command = Command::new(CARGO.lock()?.as_str());
        clippy_command.arg("clippy");
        clippy_command.arg("--workspace");

        let exit_status = clippy_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo clippy --workspace is failed");
        }
    }

    // cargo clippy --release --workspace
    {
        let mut clippy_release_command = Command::new(CARGO.lock()?.as_str());
        clippy_release_command.arg("clippy");
        clippy_release_command.arg("--release");
        clippy_release_command.arg("--workspace");

        let exit_status = clippy_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo clippy --release --workspace is failed");
        }
    }

    tracing::info!("Finished.");
    Ok(())
}

#[tracing::instrument]
fn test() -> Result {
    tracing::info!("Running...");

    // cargo test --workspace
    {
        let mut test_command = Command::new(CARGO.lock()?.as_str());
        test_command.arg("test");
        test_command.arg("--workspace");

        let exit_status = test_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo test --workspace is failed");
        }
    }

    // cargo test --release --workspace
    {
        let mut test_release_command = Command::new(CARGO.lock()?.as_str());
        test_release_command.arg("test");
        test_release_command.arg("--release");
        test_release_command.arg("--workspace");

        let exit_status = test_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo test --release --workspace is failed");
        }
    }

    tracing::info!("Finished.");
    Ok(())
}

#[tracing::instrument]
fn doc() -> Result {
    tracing::info!("Running...");

    // cargo doc --workspace
    {
        let mut doc_command = Command::new(CARGO.lock()?.as_str());
        doc_command.arg("doc");
        doc_command.arg("--workspace");

        let exit_status = doc_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo doc --workspace is failed");
        }
    }

    // cargo doc --release --workspace
    {
        let mut doc_release_command = Command::new(CARGO.lock()?.as_str());
        doc_release_command.arg("doc");
        doc_release_command.arg("--release");
        doc_release_command.arg("--workspace");

        let exit_status = doc_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo doc --release --workspace is failed");
        }
    }

    tracing::info!("Finished.");
    Ok(())
}
