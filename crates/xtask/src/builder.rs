use std::process::{Command, ExitStatus};

use crate::cli::{Action, CLIArgs};
use crate::task::{CargoInvocation, Profile, Step};
use thiserror::Error;

/// Runs the selected steps with the selected profiles, one cargo invocation at a time.
#[derive(Debug)]
pub struct SpaceroboBuilder {
    action: Action,
    steps: Vec<Step>,
    profiles: Vec<Profile>,
    cargo: String,
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

impl SpaceroboBuilder {
    pub fn new(args: CLIArgs, cargo: String) -> Self {
        Self {
            steps: Step::for_action(&args.action),
            profiles: Profile::selected(args.debug, args.release),
            action: args.action,
            cargo,
        }
    }

    /// Runs every step in order, stopping at the first failure.
    pub fn run(&self) -> Result<(), SpaceroboBuilderError> {
        let _span = tracing::info_span!("run", action = %self.action).entered();
        tracing::info!("Running...");

        for &step in &self.steps {
            self.run_step(step)?;
        }

        tracing::info!("Finished.");
        Ok(())
    }

    /// The cargo invocations of `step`, one per selected profile that the step runs in.
    fn invocations(&self, step: Step) -> impl Iterator<Item = CargoInvocation> + '_ {
        self.profiles
            .iter()
            .filter(move |&&profile| runs_in(step, profile))
            .map(move |&profile| CargoInvocation { step, profile })
    }

    fn run_step(&self, step: Step) -> Result<(), SpaceroboBuilderError> {
        let _span = tracing::info_span!("step", %step).entered();
        tracing::info!("Running...");

        for invocation in self.invocations(step) {
            self.run_cargo(&invocation)?;
        }

        tracing::info!("Finished.");
        Ok(())
    }

    fn run_cargo(&self, invocation: &CargoInvocation) -> Result<(), SpaceroboBuilderError> {
        let description = invocation.description();
        let _span = tracing::debug_span!("cargo", command = %description).entered();
        tracing::debug!("Running...");

        let status = Command::new(self.cargo.as_str())
            .args(invocation.args())
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

/// Whether `step` runs in `profile`.
/// The documentation is the same in every profile, so `doc` runs only in the debug profile.
fn runs_in(step: Step, profile: Profile) -> bool {
    !(step == Step::Doc && profile == Profile::Release)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn builder(cli: &[&str], cargo: &str) -> SpaceroboBuilder {
        let args = CLIArgs::parse_from(std::iter::once("xtask").chain(cli.iter().copied()));
        SpaceroboBuilder::new(args, cargo.to_string())
    }

    /// Every invocation `run` makes, in order.
    fn plan(builder: &SpaceroboBuilder) -> Vec<Vec<&'static str>> {
        builder
            .steps
            .iter()
            .flat_map(|&step| builder.invocations(step))
            .map(|invocation| invocation.args())
            .collect()
    }

    #[test]
    fn runs_each_step_with_every_profile_before_the_next_step() {
        let plan = plan(&builder(&["all", "--release", "--debug"], "cargo"));

        let subcommands: Vec<(&str, bool)> = plan
            .iter()
            .map(|args| (args[0], args.contains(&"--release")))
            .collect();
        assert_eq!(
            subcommands,
            [
                ("build", false),
                ("build", true),
                ("check", false),
                ("check", true),
                ("clippy", false),
                ("clippy", true),
                ("test", false),
                ("test", true),
                ("doc", false),
            ]
        );
    }

    #[test]
    fn runs_every_step_in_the_debug_profile_by_default() {
        let plan = plan(&builder(&[], "cargo"));

        let mut expected = ["build", "check", "clippy", "test", "doc"]
            .map(|subcommand| vec![subcommand, "--workspace", "--exclude", "spacerobo_xtask"]);
        expected[4].push("--no-deps");
        assert_eq!(plan, expected);
    }

    #[test]
    fn skips_doc_in_the_release_profile() {
        assert_eq!(
            plan(&builder(&["doc", "--debug", "--release"], "cargo")),
            [[
                "doc",
                "--workspace",
                "--exclude",
                "spacerobo_xtask",
                "--no-deps"
            ]]
        );
        assert!(plan(&builder(&["doc", "--release"], "cargo")).is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn succeeds_when_the_command_succeeds() {
        assert!(builder(&["build", "-d", "-r"], "true").run().is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn returns_an_error_when_the_command_fails() {
        let error = builder(&["build", "-d", "-r"], "false").run().unwrap_err();

        assert!(matches!(error, SpaceroboBuilderError::CommandFailed { .. }));
    }

    #[cfg(unix)]
    #[test]
    fn returns_an_error_when_the_command_is_missing() {
        let error = builder(&["build", "-d", "-r"], "spacerobo-no-such-command")
            .run()
            .unwrap_err();

        assert!(matches!(error, SpaceroboBuilderError::Io { .. }));
    }
}
