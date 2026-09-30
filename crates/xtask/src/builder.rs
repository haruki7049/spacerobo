use std::process::{Command, ExitStatus};

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
    #[error("Failed to run `{command}`: {source}")]
    Io {
        command: String,
        #[source]
        source: std::io::Error,
    },

    #[error("`{command}` failed with {status}")]
    CommandFailed { command: String, status: ExitStatus },
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
        self.run_for_targets("build")
    }

    #[tracing::instrument]
    fn check(&self) -> Result<(), Self::Error> {
        self.run_for_targets("check")
    }

    #[tracing::instrument]
    fn clippy(&self) -> Result<(), Self::Error> {
        self.run_for_targets("clippy")
    }

    #[tracing::instrument]
    fn test(&self) -> Result<(), Self::Error> {
        self.run_for_targets("test")
    }

    #[tracing::instrument]
    fn doc(&self) -> Result<(), Self::Error> {
        self.run_for_targets("doc")
    }
}

impl SpaceroboBuilder {
    /// Runs the cargo subcommand for every selected build target (debug and/or release).
    fn run_for_targets(&self, subcommand: &str) -> Result<(), SpaceroboBuilderError> {
        tracing::info!("Running...");

        if self.is_debug() {
            self.run_cargo(subcommand, false)?;
        }
        if self.is_release() {
            self.run_cargo(subcommand, true)?;
        }

        tracing::info!("Finished.");
        Ok(())
    }

    /// `cargo <subcommand> [--release] --workspace --exclude spacerobo_xtask`
    #[tracing::instrument]
    fn run_cargo(&self, subcommand: &str, release: bool) -> Result<(), SpaceroboBuilderError> {
        tracing::debug!("Running...");

        let mut command = Command::new(self.cargo.as_str());
        command.arg(subcommand);
        if release {
            command.arg("--release");
        }
        command.args(["--workspace", "--exclude", "spacerobo_xtask"]);

        let description = if release {
            format!("cargo {subcommand} --release --workspace")
        } else {
            format!("cargo {subcommand} --workspace")
        };

        let status = command
            .status()
            .map_err(|source| SpaceroboBuilderError::Io {
                command: description.clone(),
                source,
            })?;

        if !status.success() {
            return Err(SpaceroboBuilderError::CommandFailed {
                command: description,
                status,
            });
        }

        tracing::debug!("Finished.");
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

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn builder(cargo: &str) -> SpaceroboBuilder {
        SpaceroboBuilder {
            targets: vec![BuildTarget::Debug, BuildTarget::Release],
            action: Action::Build,
            cargo: cargo.to_string(),
        }
    }

    #[test]
    fn succeeds_when_the_command_succeeds() {
        assert!(builder("true").build().is_ok());
    }

    #[test]
    fn returns_an_error_when_the_command_fails() {
        let error = builder("false").build().unwrap_err();

        assert!(matches!(error, SpaceroboBuilderError::CommandFailed { .. }));
    }

    #[test]
    fn returns_an_error_when_the_command_is_missing() {
        let error = builder("spacerobo-no-such-command").build().unwrap_err();

        assert!(matches!(error, SpaceroboBuilderError::Io { .. }));
    }
}
