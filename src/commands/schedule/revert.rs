use clap::Args;

use crate::{Handle, Index, Schedule};

#[derive(Debug, Args)]
pub struct RevertCommand {
    hash: String,
}

pub fn evaluate(index: &mut Index, args: RevertCommand) {
    if args.hash == "ROOT" {
        index.head = None;
        index.write();
        return;
    }

    let handle: Handle<Schedule> = Handle::parse(&args.hash);

    handle.resolve().expect("could not resolve handle");
    index.head = Some(handle);
}
