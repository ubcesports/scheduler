// use clap::Args;
// use std::fs;
//
// use crate::{Context, Handle, Referential};
//
// #[derive(Debug, Args)]
// pub struct ExportCommand {
//     schedule: Option<String>,
//     #[arg(long, default_value_t = false)]
//     json: bool,
// }
//
// pub async fn evaluate(ctx: &Context, args: ExportCommand) {
//     let schedule = args
//         .schedule
//         .map(|hash| Handle::parse(&hash))
//         .unwrap_or(ctx.index.head.expect("no schedule to show"))
//         .resolve()
//         .expect("could not resolve schedule");
//
//     if args.json {
//         fs::write(
//             format!("{}.json", schedule.handle().to_string()),
//             serde_json::to_string(&schedule).expect("could not serialize json"),
//         )
//         .expect("could not write json");
//
//         return;
//     }
//
//     let mut slots = Vec::from_iter(ctx.index.slots.iter());
//     slots.sort();
//
//     let mut output = String::new();
//
//     for (i, slot) in Iterator::enumerate(slots.into_iter()) {
//         let res = schedule.get_slot(*slot);
//
//         output += &format!(
//             "{}\n{}\n{}\n{}\n",
//             res[0].resolve().unwrap().name(),
//             res[1].resolve().unwrap().name(),
//             res[0].resolve().unwrap().name(),
//             res[1].resolve().unwrap().name()
//         );
//
//         if i % 5 == 4 {
//             output += "\n";
//         }
//     }
//
//     fs::write(format!("{}.csv", schedule.handle().to_string()), output)
//         .expect("could not write exported csv");
// }
