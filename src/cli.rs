use clap::{Args, Parser, Subcommand, ValueEnum};

/// CLI Template with Ratatui and Clap
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Launch interactive TUI mode
    #[command(visible_alias = "ui")]
    Tui(TuiArgs),

    /// Print a greeting message
    Greet(GreetArgs),
}

/// Arguments for TUI subcommand
#[derive(Args, Debug)]
pub struct TuiArgs {
    /// Tick rate in milliseconds
    #[arg(long, default_value_t = 250)]
    pub tick_rate: u64,
}

/// Arguments for Greet subcommand
#[derive(Args, Debug)]
pub struct GreetArgs {
    /// Name to greet
    #[arg(default_value = "World")]
    pub name: String,

    /// Number of times to greet
    #[arg(short = 'n', long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=10))]
    pub count: u8,

    /// Greeting style
    #[arg(short, long, value_enum, default_value_t = GreetStyle::Normal)]
    pub style: GreetStyle,
}

/// Greeting styles
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum GreetStyle {
    Normal,
    Formal,
    Casual,
}
