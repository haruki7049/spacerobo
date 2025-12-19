use bevy::prelude::*;
use clap::Parser;
use directories::ProjectDirs;
use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

/// Command-Line Arguments, using clap crate
/// This structure allows users to set an additional configuration file
#[derive(Parser, Debug)]
#[clap(version, author, about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct CLIArgs {
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH.lock().unwrap().display().to_string())]
    config_file: PathBuf,
}

impl CLIArgs {
    pub fn config_file(&self) -> PathBuf {
        self.config_file.clone()
    }
}

/// Default Configuration Path, using directories crate to calculate ProjectDirs (~/.config/spacerobo)
static DEFAULT_CONFIG_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| {
    let proj_dirs = ProjectDirs::from("dev", "haruki7049", "spacerobo")
        .expect("Failed to search ProjectDirs for dev.haruki7049.spacerobo");
    let mut config_path: PathBuf = proj_dirs.config_dir().to_path_buf();
    let filename: &str = "config.toml";

    config_path.push(filename);
    Mutex::new(config_path)
});

#[cfg(test)]
mod tests {
    mod cli_args {
        use crate::cli::CLIArgs;
        use std::path::PathBuf;

        #[test]
        fn config_file() {
            let cli_args: CLIArgs = CLIArgs {
                config_file: PathBuf::new(),
            };

            assert_eq!(cli_args.config_file(), PathBuf::new());
        }
    }
}
