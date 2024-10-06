// use clap::Args;
//
// use crate::{Context, Handle, Referential, ScheduleOld};
//
// #[derive(Debug, Args)]
// pub struct ImportCommand {
//     hash: String,
// }
//
// pub async fn evaluate(ctx: &mut Context, args: ImportCommand) {
//     let handle: Handle<ScheduleOld> = Handle::parse(&args.hash);
//     let result = handle.resolve().unwrap().rewrite();
//
//     if ctx.index.head.is_some_and(|h| h == handle) {
//         ctx.index.head = Some(result.handle());
//     }
// }
