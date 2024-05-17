use std::fmt::Display;

use clap::{Parser, ValueEnum};

use crate::config::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub config: Config,

    #[clap(short, long, default_value_t = Mode::Terminal)]
    pub mode: Mode,
}

#[derive(Clone, ValueEnum)]
pub enum Mode {
    Terminal,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Terminal => write!(f, "terminal"),
        }
    }
}
