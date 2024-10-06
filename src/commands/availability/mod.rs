mod fetch;
mod show;
mod r#use;

use clap::{Args, Subcommand};

use crate::Context;

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

pub async fn evaluate(index: &Context, args: AvailabilityCommand) {
    match args.command {
        AvailabilitySubcommand::Use(args) => r#use::evaluate(index, args).await,
        AvailabilitySubcommand::Fetch(args) => fetch::evaluate(index, args).await,
        AvailabilitySubcommand::Show(args) => show::evaluate(index, args).await,
    }
}
