use crate::{Availability, Handle, Index};
use clap::Args;

#[derive(Debug, Args)]
pub struct UseCommand {
    hash: String,
}

pub fn evaluate(index: &mut Index, args: UseCommand) {
    let handle: Handle<Availability> = Handle::parse(&args.hash);

    let _ = handle.resolve().expect("could not resolve hash");

    index.availability = Some(handle);
}
