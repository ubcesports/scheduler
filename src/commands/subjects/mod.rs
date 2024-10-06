mod resolve;

use clap::{Args, Subcommand};

use crate::Context;

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

pub async fn evaluate(ctx: &Context, args: SubjectsCommand) {
    match args.command {
        SubjectsSubcommand::Resolve(args) => resolve::evaluate(ctx, args).await,
    }
}
