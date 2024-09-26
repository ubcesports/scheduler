use clap::{Parser, Subcommand};
use sch::{commands, Index};

#[derive(Debug, Parser)]
#[command(name = "sch", version)]
#[command(about = "Shift scheduling utility", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
#[command(name = "subcommand")]
enum Command {
    #[command()]
    Init(commands::init::InitCommand),

    #[command(arg_required_else_help = true)]
    Schedule(commands::schedule::ScheduleCommand),

    #[command(arg_required_else_help = true)]
    Availability(commands::availability::AvailabilityCommand),

    #[command(arg_required_else_help = true)]
    Subjects(commands::subjects::SubjectsCommand),
}

fn main() {
    let args = Cli::parse();

    if let Command::Init(args) = args.command {
        commands::init::evaluate(args);
        return;
    }

    let mut index = Index::load();

    match args.command {
        Command::Availability(args) => commands::availability::evaluate(&mut index, args),
        Command::Schedule(args) => commands::schedule::evaluate(&mut index, args),
        Command::Subjects(args) => commands::subjects::evaluate(&mut index, args),
        Command::Init(_) => unreachable!("already handled"),
    };

    index.write();
    println!("ok");
}
