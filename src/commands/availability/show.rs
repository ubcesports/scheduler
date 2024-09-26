use crate::Index;
use clap::Args;

#[derive(Debug, Args)]
pub struct ShowCommand;

pub fn evaluate(index: &mut Index, _args: ShowCommand) {
    let availability = index
        .availability
        .expect("no availability selected")
        .resolve()
        .expect("could not resolve availability");

    let mut slots = Vec::from_iter(index.slots.iter());
    slots.sort();

    for (i, slot) in Iterator::enumerate(slots.iter()) {
        if i % 5 == 0 {
            println!();
        }

        print!("{}\t", availability.for_slot(**slot).len());
    }
}
