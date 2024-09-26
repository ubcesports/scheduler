mod fetch;
mod show;
mod r#use;

use clap::{Args, Subcommand};

use crate::Index;

#[derive(Debug, Args)]
#[command(about = "Manage subject availability")]
pub struct AvailabilityCommand {
    #[command(subcommand)]
    command: AvailabilitySubcommand,
}

#[derive(Debug, Subcommand)]
enum AvailabilitySubcommand {
    Fetch(fetch::FetchCommand),
    Use(r#use::UseCommand),
    Show(show::ShowCommand),
}

pub fn evaluate(index: &mut Index, args: AvailabilityCommand) {
    match args.command {
        AvailabilitySubcommand::Use(args) => r#use::evaluate(index, args),
        AvailabilitySubcommand::Fetch(args) => fetch::evaluate(index, args),
        AvailabilitySubcommand::Show(args) => show::evaluate(index, args),
    }
}
