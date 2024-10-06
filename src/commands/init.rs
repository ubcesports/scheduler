use std::{env, path::PathBuf};

use clap::Args;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;

#[derive(Debug, Args)]
#[command(about = "Initialize a new project")]
pub struct InitCommand {
    path: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    force: bool,
}

pub async fn evaluate(args: InitCommand) {
    let path = args.path.unwrap_or(
        env::current_dir()
            .unwrap_or(Default::default())
            .join("sched.db"),
    );

    if path.exists() && !args.force {
        println!("index already exists! use --force to rebuild index!");
        return;
    }

    SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true),
    )
    .await
    .expect("could not create new database");
}
