use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "learning-programming-app")]
#[command(about = "A file-watching automatic code execution CLI for programming learning")]
#[command(version = "0.1.0")]
#[command(author = "Learning Programming App Team")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start watching a directory for file changes
    Watch {
        /// Directory to watch for changes
        #[arg(short, long, default_value = "./examples")]
        directory: PathBuf,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// List available sections
    Sections {
        /// Directory containing sections
        #[arg(short, long, default_value = "./examples")]
        directory: PathBuf,
    },

    /// Show execution history
    History {
        /// Number of recent executions to show
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },

    /// Show execution statistics
    Stats,

    /// Clear execution history
    Clear {
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Execute a specific file once
    Run {
        /// File path to execute
        file: PathBuf,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Generate Go learning problems with section preview
    GenerateGo {
        /// Skip section preview and use defaults
        #[arg(short, long)]
        skip_preview: bool,

        /// Output directory for generated problems
        #[arg(short, long, default_value = "./learning-go")]
        output: PathBuf,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }

    #[test]
    fn test_default_watch_command() {
        let cli = Cli::try_parse_from(&["learning-programming-app", "watch"]).unwrap();

        if let Some(Commands::Watch { directory, verbose }) = cli.command {
            assert_eq!(directory, PathBuf::from("./examples"));
            assert!(!verbose);
        } else {
            panic!("Expected Watch command");
        }
    }

    #[test]
    fn test_sections_command() {
        let cli = Cli::try_parse_from(&["learning-programming-app", "sections"]).unwrap();

        if let Some(Commands::Sections { directory }) = cli.command {
            assert_eq!(directory, PathBuf::from("./examples"));
        } else {
            panic!("Expected Sections command");
        }
    }

    #[test]
    fn test_history_command() {
        let cli =
            Cli::try_parse_from(&["learning-programming-app", "history", "--limit", "5"]).unwrap();

        if let Some(Commands::History { limit }) = cli.command {
            assert_eq!(limit, 5);
        } else {
            panic!("Expected History command");
        }
    }

    #[test]
    fn test_generate_go_command() {
        let cli = Cli::try_parse_from(&["learning-programming-app", "generate-go"]).unwrap();

        if let Some(Commands::GenerateGo {
            skip_preview,
            output,
        }) = cli.command
        {
            assert!(!skip_preview);
            assert_eq!(output, PathBuf::from("./learning-go"));
        } else {
            panic!("Expected GenerateGo command");
        }
    }

    #[test]
    fn test_generate_go_command_with_options() {
        let cli = Cli::try_parse_from(&[
            "learning-programming-app",
            "generate-go",
            "--skip-preview",
            "--output",
            "./custom-go",
        ])
        .unwrap();

        if let Some(Commands::GenerateGo {
            skip_preview,
            output,
        }) = cli.command
        {
            assert!(skip_preview);
            assert_eq!(output, PathBuf::from("./custom-go"));
        } else {
            panic!("Expected GenerateGo command");
        }
    }
}
