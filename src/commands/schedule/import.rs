use clap::Args;

use crate::{Handle, Index, Referential, Schedule};

#[derive(Debug, Args)]
pub struct ImportCommand {
    hash: String,
}

pub fn evaluate(index: &mut Index, args: ImportCommand) {
    let handle: Handle<Schedule> = Handle::parse(&args.hash);
    let result = handle.resolve().unwrap().rewrite();

    if index.head.is_some_and(|h| h == handle) {
        index.head = Some(result.handle());
    }
}
