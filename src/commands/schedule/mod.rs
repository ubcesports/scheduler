// mod export;
mod generate;
// mod import;
mod revert;
mod show;

use crate::Context;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(about = "Manage schedules")]
pub struct ScheduleCommand {
    #[command(subcommand)]
    command: ScheduleSubcommand,
}

#[derive(Debug, Subcommand)]
enum ScheduleSubcommand {
    Generate(generate::GenerateCommand),
    Show(show::ShowCommand),
    // Export(export::ExportCommand),
    Revert(revert::RevertCommand),
    // Import(import::ImportCommand),
}

pub async fn evaluate(ctx: &mut Context, args: ScheduleCommand) {
    match args.command {
        ScheduleSubcommand::Generate(args) => generate::evaluate(ctx, args).await,
        ScheduleSubcommand::Show(args) => show::evaluate(ctx, args).await,
        // ScheduleSubcommand::Export(args) => export::evaluate(ctx, args).await,
        ScheduleSubcommand::Revert(args) => revert::evaluate(ctx, args).await,
        // ScheduleSubcommand::Import(args) => import::evaluate(ctx, args).await,
    }
}
