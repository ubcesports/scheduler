mod resolve;

use clap::{Args, Subcommand};

use crate::Index;

#[derive(Debug, Args)]
#[command(about = "Manage subjects")]
pub struct SubjectsCommand {
    #[command(subcommand)]
    command: SubjectsSubcommand,
}

#[derive(Debug, Subcommand)]
enum SubjectsSubcommand {
    Resolve(resolve::ResolveCommand),
}

pub fn evaluate(index: &mut Index, args: SubjectsCommand) {
    match args.command {
        SubjectsSubcommand::Resolve(args) => resolve::evaluate(index, args),
    }
}
