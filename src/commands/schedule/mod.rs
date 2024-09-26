mod export;
mod generate;
mod import;
mod revert;
mod show;

use crate::Index;
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
    Export(export::ExportCommand),
    Revert(revert::RevertCommand),
    Import(import::ImportCommand),
}

pub fn evaluate(index: &mut Index, args: ScheduleCommand) {
    match args.command {
        ScheduleSubcommand::Generate(args) => generate::evaluate(index, args),
        ScheduleSubcommand::Show(args) => show::evaluate(index, args),
        ScheduleSubcommand::Export(args) => export::evaluate(index, args),
        ScheduleSubcommand::Revert(args) => revert::evaluate(index, args),
        ScheduleSubcommand::Import(args) => import::evaluate(index, args),
    }
}
