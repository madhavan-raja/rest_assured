use clap::Parser;
mod cli;
mod config;
mod tui;

use cli::Cli;
use tui::Tui;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let config = args.config;
    let mode = args.mode;

    match mode {
        cli::Mode::Terminal => {
            let terminal = Tui::new(config);
            terminal.start()?;
        }
    }

    Ok(())
}
