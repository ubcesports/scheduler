use clap::{Parser, Subcommand};
use sch::{commands, Context};
use sqlx::PgPool;

#[derive(Debug, Parser)]
#[command(name = "sch", version)]
#[command(about = "Shift scheduling utility", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long)]
    db: String,
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

    #[command()]
    Migrate(commands::migrate::MigrateCommand),
}

#[tokio::main]
pub async fn main() {
    let args = Cli::parse();

    if let Command::Init(args) = args.command {
        commands::init::evaluate(args).await;
        return;
    }

    let db = PgPool::connect(&args.db).await.unwrap();

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("could not run migrations");

    let mut context = Context { db };

    match args.command {
        Command::Availability(args) => commands::availability::evaluate(&mut context, args).await,
        Command::Schedule(args) => commands::schedule::evaluate(&mut context, args).await,
        Command::Subjects(args) => commands::subjects::evaluate(&mut context, args).await,
        Command::Migrate(args) => commands::migrate::evaluate(&mut context, args).await,
        Command::Init(_) => unreachable!("already handled"),
    };
}
