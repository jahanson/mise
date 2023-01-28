extern crate core;
#[macro_use]
extern crate log;

use color_eyre::eyre::Result;

use crate::cli::Cli;
use crate::config::Config;
use crate::output::Output;

#[macro_use]
mod output;

#[macro_use]
mod regex;

pub mod build_time;
mod cli;
mod cmd;
mod config;
mod dirs;
mod env;
mod env_diff;
mod errors;
mod fake_asdf;
mod file;
mod git;
mod hook_env;
mod logger;
mod plugins;
pub mod runtimes;
mod shell;
pub mod shorthand_repository;
mod ui;

mod hash;
#[cfg(test)]
mod test;

fn main() -> Result<()> {
    if hook_env::should_exit_early(env::vars().collect()) {
        return Ok(());
    }
    color_eyre::install()?;

    let log_level = *env::RTX_LOG_LEVEL;
    logger::init(log_level);
    let config = Config::load()?;
    let cli = Cli::new_with_external_commands(&config)?;

    match cli.run(config, &env::ARGS, &mut Output::new()) {
        Err(err) if log_level < log::LevelFilter::Debug => {
            eprintln!("rtx: error {err}");
            eprintln!(
                // "rtx error: Run with `--log-level debug` or RTX_DEBUG=1 for more information."
                "rtx: Run with RTX_DEBUG=1 for more information."
            );
            std::process::exit(1);
        }
        result => result,
    }
}
