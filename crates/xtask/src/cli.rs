use clap::{Parser, Subcommand};
use thiserror::Error;

#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct CLIArgs {
    #[clap(default_value_t = Action::All)]
    pub action: Action,

    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    #[clap(short, long, default_value_t = false)]
    pub release: bool,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Action {
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
    type Err = XtaskError;

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
pub enum XtaskError {
    ParseError(String),
}

impl std::fmt::Display for XtaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(s) => write!(f, "ParseError by \"{}\"", s),
        }
    }
}
