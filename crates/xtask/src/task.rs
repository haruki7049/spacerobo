//! The units of work that xtask runs: steps, profiles and the cargo invocations built from them.

use crate::cli::Action;

/// Arguments that make every invocation cover the workspace except xtask itself.
const WORKSPACE_ARGS: [&str; 3] = ["--workspace", "--exclude", "spacerobo_xtask"];

/// A cargo subcommand that xtask runs over the workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Build,
    Check,
    Clippy,
    Test,
    Doc,
}

impl Step {
    /// Every step, in the order `all` runs them.
    pub const ALL: [Step; 5] = [
        Step::Build,
        Step::Check,
        Step::Clippy,
        Step::Test,
        Step::Doc,
    ];

    /// The steps that `action` runs, in order.
    pub fn for_action(action: &Action) -> Vec<Step> {
        match action {
            Action::All => Self::ALL.to_vec(),
            Action::Build => vec![Step::Build],
            Action::Check => vec![Step::Check],
            Action::Clippy => vec![Step::Clippy],
            Action::Test => vec![Step::Test],
            Action::Doc => vec![Step::Doc],
        }
    }

    pub fn subcommand(self) -> &'static str {
        match self {
            Step::Build => "build",
            Step::Check => "check",
            Step::Clippy => "clippy",
            Step::Test => "test",
            Step::Doc => "doc",
        }
    }
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.subcommand())
    }
}

/// A cargo build profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Debug,
    Release,
}

impl Profile {
    /// The profiles selected by the `--debug` and `--release` flags, debug first.
    /// Without either flag, only the debug profile is selected.
    pub fn selected(debug: bool, release: bool) -> Vec<Profile> {
        let mut profiles = Vec::new();
        if debug || !release {
            profiles.push(Profile::Debug);
        }
        if release {
            profiles.push(Profile::Release);
        }
        profiles
    }

    fn cargo_args(self) -> &'static [&'static str] {
        match self {
            Profile::Debug => &[],
            Profile::Release => &["--release"],
        }
    }
}

/// One cargo command: a step run with a profile.
///
/// Options that change the arguments of a step or a profile belong in [`CargoInvocation::args`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CargoInvocation {
    pub step: Step,
    pub profile: Profile,
}

impl CargoInvocation {
    /// The arguments passed to cargo: `<subcommand> [--release] --workspace --exclude spacerobo_xtask`.
    pub fn args(&self) -> Vec<&'static str> {
        let mut args = vec![self.step.subcommand()];
        args.extend_from_slice(self.profile.cargo_args());
        args.extend_from_slice(&WORKSPACE_ARGS);
        args
    }

    /// The short form of the command used in error messages: `cargo <subcommand> [--release] --workspace`.
    pub fn description(&self) -> String {
        let mut words = vec!["cargo", self.step.subcommand()];
        words.extend_from_slice(self.profile.cargo_args());
        words.push("--workspace");
        words.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_runs_every_step_in_order() {
        assert_eq!(
            Step::for_action(&Action::All),
            vec![
                Step::Build,
                Step::Check,
                Step::Clippy,
                Step::Test,
                Step::Doc
            ]
        );
    }

    #[test]
    fn a_single_action_runs_only_its_step() {
        assert_eq!(Step::for_action(&Action::Build), vec![Step::Build]);
        assert_eq!(Step::for_action(&Action::Check), vec![Step::Check]);
        assert_eq!(Step::for_action(&Action::Clippy), vec![Step::Clippy]);
        assert_eq!(Step::for_action(&Action::Test), vec![Step::Test]);
        assert_eq!(Step::for_action(&Action::Doc), vec![Step::Doc]);
    }

    #[test]
    fn debug_is_the_default_profile() {
        assert_eq!(Profile::selected(false, false), vec![Profile::Debug]);
        assert_eq!(Profile::selected(true, false), vec![Profile::Debug]);
        assert_eq!(Profile::selected(false, true), vec![Profile::Release]);
        assert_eq!(
            Profile::selected(true, true),
            vec![Profile::Debug, Profile::Release]
        );
    }

    #[test]
    fn args_cover_the_workspace_except_xtask() {
        let debug = CargoInvocation {
            step: Step::Doc,
            profile: Profile::Debug,
        };
        let release = CargoInvocation {
            step: Step::Clippy,
            profile: Profile::Release,
        };

        assert_eq!(
            debug.args(),
            ["doc", "--workspace", "--exclude", "spacerobo_xtask"]
        );
        assert_eq!(
            release.args(),
            [
                "clippy",
                "--release",
                "--workspace",
                "--exclude",
                "spacerobo_xtask"
            ]
        );
    }

    #[test]
    fn description_is_the_short_form_of_the_command() {
        let debug = CargoInvocation {
            step: Step::Build,
            profile: Profile::Debug,
        };
        let release = CargoInvocation {
            step: Step::Test,
            profile: Profile::Release,
        };

        assert_eq!(debug.description(), "cargo build --workspace");
        assert_eq!(release.description(), "cargo test --release --workspace");
    }
}
