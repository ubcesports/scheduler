use std::{env, fs};

use clap::Args;

use crate::{Handle, Index, Referential, Subject};

#[derive(Debug, Args)]
pub struct ResolveCommand {
    name: String,
}

pub fn evaluate(_index: &mut Index, args: ResolveCommand) {
    let subjects = fs::read_dir(
        env::current_dir()
            .unwrap_or(Default::default())
            .join("subjects"),
    )
    .into_iter()
    .map(|read_dir| read_dir.collect::<Vec<_>>())
    .flatten()
    .filter_map(|entry| entry.ok())
    .map(|e| e.file_name())
    .filter_map(|f| f.into_string().ok())
    .filter_map(|f| f.split_once(".").map(|(a, _)| a.to_owned()))
    .filter_map(|f| Handle::<Subject>::parse(&f).resolve().ok())
    .filter(|subject| subject.name() == args.name)
    .collect::<Vec<_>>();

    if let Some(subject) = subjects.first() {
        println!("{}: {}\n{}", subject.id(), subject.name(), subject.handle());
    } else {
        println!("could not find subject");
    }
}
