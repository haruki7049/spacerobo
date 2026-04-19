use std::process::Command;

use crate::cli::{Action, CLIArgs};
use thiserror::Error;

#[derive(Debug)]
pub struct SpaceroboBuilder {
    targets: Vec<BuildTarget>,
    action: Action,
    cargo: String,
}

#[derive(Debug, PartialEq)]
enum BuildTarget {
    Debug,
    Release,
}

impl SpaceroboBuilder {
    pub fn new(args: CLIArgs, cargo: String) -> Self {
        let mut targets: Vec<BuildTarget> = Vec::new();
        if !args.debug && !args.release {
            targets.push(BuildTarget::Debug);
        }

        if args.debug {
            targets.push(BuildTarget::Debug);
        }
        if args.release {
            targets.push(BuildTarget::Release);
        }

        Self {
            targets,
            cargo,
            action: args.action,
        }
    }
}

impl SpaceroboBuilder {
    fn is_debug(&self) -> bool {
        let search_result: Option<&BuildTarget> =
            self.targets.iter().find(|&v| v == &BuildTarget::Debug);

        search_result.is_some()
    }

    fn is_release(&self) -> bool {
        let search_result: Option<&BuildTarget> =
            self.targets.iter().find(|&v| v == &BuildTarget::Release);

        search_result.is_some()
    }
}

#[derive(Debug, Error)]
pub enum SpaceroboBuilderError {
    #[error("From any error: {0:?}")]
    Error(#[from] Box<dyn std::error::Error>),
}

impl Builder for SpaceroboBuilder {
    type Error = SpaceroboBuilderError;

    #[tracing::instrument]
    fn action(&self) -> Action {
        self.action.clone()
    }

    #[tracing::instrument]
    fn run(&self) -> Result<(), Self::Error> {
        match self.action() {
            Action::All => self.all(),
            Action::Build => self.build(),
            Action::Check => self.check(),
            Action::Clippy => self.clippy(),
            Action::Test => self.test(),
            Action::Doc => self.doc(),
        }
    }

    #[tracing::instrument]
    fn all(&self) -> Result<(), Self::Error> {
        tracing::info!("Running...");
        self.build()?;
        self.check()?;
        self.clippy()?;
        self.test()?;
        self.doc()?;
        tracing::info!("Finished.");

        Ok(())
    }

    #[tracing::instrument]
    fn build(&self) -> Result<(), Self::Error> {
        tracing::info!("Running...");

        if self.is_debug() {
            self.build_debug()?;
        }
        if self.is_release() {
            self.build_release()?;
        }

        tracing::info!("Finished.");
        Ok(())
    }

    #[tracing::instrument]
    fn check(&self) -> Result<(), Self::Error> {
        tracing::info!("Running...");

        if self.is_debug() {
            self.check_debug()?;
        }
        if self.is_release() {
            self.check_release()?;
        }

        tracing::info!("Finished.");
        Ok(())
    }

    #[tracing::instrument]
    fn clippy(&self) -> Result<(), Self::Error> {
        tracing::info!("Running...");

        if self.is_debug() {
            self.clippy_debug()?;
        }
        if self.is_release() {
            self.clippy_release()?;
        }

        tracing::info!("Finished.");
        Ok(())
    }

    #[tracing::instrument]
    fn test(&self) -> Result<(), Self::Error> {
        tracing::info!("Running...");

        if self.is_debug() {
            self.test_debug()?;
        }
        if self.is_release() {
            self.test_release()?;
        }

        tracing::info!("Finished.");
        Ok(())
    }

    #[tracing::instrument]
    fn doc(&self) -> Result<(), Self::Error> {
        tracing::info!("Running...");

        if self.is_debug() {
            self.doc_debug()?;
        }
        if self.is_release() {
            self.doc_release()?;
        }

        tracing::info!("Finished.");
        Ok(())
    }
}

impl SpaceroboBuilder {
    /// `cargo build --release --workspace`
    #[tracing::instrument]
    fn build_release(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut build_release_command = Command::new(self.cargo.as_str());
        build_release_command.arg("build");
        build_release_command.arg("--release");
        build_release_command.arg("--workspace");

        let exit_status = build_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo build --release --workspace is failed");
        }

        tracing::debug!("Finished.");
        Ok(())
    }

    /// `cargo build --workspace`
    #[tracing::instrument]
    fn build_debug(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut build_command = Command::new(self.cargo.as_str());
        build_command.arg("build");
        build_command.arg("--workspace");

        let exit_status = build_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo build --workspace is failed");
        }

        tracing::debug!("Finished.");
        Ok(())
    }

    /// `cargo check --release --workspace`
    #[tracing::instrument]
    fn check_release(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut check_release_command = Command::new(self.cargo.as_str());
        check_release_command.arg("check");
        check_release_command.arg("--release");
        check_release_command.arg("--workspace");

        let exit_status = check_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo check --release --workspace is failed");
        }

        tracing::debug!("Finished.");
        Ok(())
    }

    /// `cargo check --workspace`
    #[tracing::instrument]
    fn check_debug(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut check_command = Command::new(self.cargo.as_str());
        check_command.arg("check");
        check_command.arg("--workspace");

        let exit_status = check_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo check --workspace is failed");
        }

        tracing::debug!("Finished.");
        Ok(())
    }

    /// `cargo clippy --release --workspace`
    #[tracing::instrument]
    fn clippy_release(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut clippy_release_command = Command::new(self.cargo.as_str());
        clippy_release_command.arg("clippy");
        clippy_release_command.arg("--release");
        clippy_release_command.arg("--workspace");

        let exit_status = clippy_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo clippy --release --workspace is failed");
        }

        tracing::debug!("Finished.");
        Ok(())
    }

    /// `cargo clippy --workspace`
    #[tracing::instrument]
    fn clippy_debug(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut clippy_command = Command::new(self.cargo.as_str());
        clippy_command.arg("clippy");
        clippy_command.arg("--workspace");

        let exit_status = clippy_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo clippy --workspace is failed");
        }

        tracing::debug!("Finished.");
        Ok(())
    }

    /// `cargo test --release --workspace`
    #[tracing::instrument]
    fn test_release(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut test_release_command = Command::new(self.cargo.as_str());
        test_release_command.arg("test");
        test_release_command.arg("--release");
        test_release_command.arg("--workspace");

        let exit_status = test_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo test --release --workspace is failed");
        }

        tracing::debug!("Finished.");
        Ok(())
    }

    /// `cargo test --workspace`
    #[tracing::instrument]
    fn test_debug(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut test_command = Command::new(self.cargo.as_str());
        test_command.arg("test");
        test_command.arg("--workspace");

        let exit_status = test_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo test --workspace is failed");
        }

        tracing::debug!("Finished.");
        Ok(())
    }

    /// `cargo doc --workspace`
    #[tracing::instrument]
    fn doc_debug(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut doc_command = Command::new(self.cargo.as_str());
        doc_command.arg("doc");
        doc_command.arg("--workspace");

        let exit_status = doc_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo doc --workspace is failed");
        }

        tracing::info!("Finished.");
        Ok(())
    }

    /// `cargo doc --release --workspace`
    #[tracing::instrument]
    fn doc_release(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Running...");

        let mut doc_release_command = Command::new(self.cargo.as_str());
        doc_release_command.arg("doc");
        doc_release_command.arg("--release");
        doc_release_command.arg("--workspace");

        let exit_status = doc_release_command.spawn()?.wait()?;

        if !exit_status.success() {
            panic!("cargo doc --release --workspace is failed");
        }

        tracing::info!("Finished.");
        Ok(())
    }
}

pub trait Builder {
    type Error;

    fn action(&self) -> Action;
    fn run(&self) -> Result<(), Self::Error>;
    fn all(&self) -> Result<(), Self::Error>;
    fn build(&self) -> Result<(), Self::Error>;
    fn check(&self) -> Result<(), Self::Error>;
    fn clippy(&self) -> Result<(), Self::Error>;
    fn test(&self) -> Result<(), Self::Error>;
    fn doc(&self) -> Result<(), Self::Error>;
}
