use std::{env, path::PathBuf};

use clap::Args;

use crate::Index;

#[derive(Debug, Args)]
#[command(about = "Initialize a new project")]
pub struct InitCommand {
    path: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    force: bool,
}

pub fn evaluate(args: InitCommand) {
    let path = args
        .path
        .unwrap_or(env::current_dir().unwrap_or(Default::default()));

    let index_path = path.join("index.sch");

    if index_path.exists() && !args.force {
        println!("index already exists! use --force to rebuild index!");
        return;
    }

    Index::new().write_to(index_path);
}
