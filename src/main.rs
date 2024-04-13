use clap::Parser;
mod cli;
mod config;
mod terminal_mode;

use cli::Cli;
use terminal_mode::TerminalMode;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let config = args.config;
    let mode = args.mode;

    match mode {
        cli::Mode::Terminal => {
            let terminal_mode = TerminalMode::new(config);
            terminal_mode.start()?;
        }
        cli::Mode::Web => println!("Running in web mode"),
    }

    Ok(())
}
