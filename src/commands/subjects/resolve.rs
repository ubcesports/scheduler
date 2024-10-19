use crate::Context;
use clap::Args;

#[derive(Debug, Args)]
pub struct ResolveCommand {
    name: String,
}

pub async fn evaluate(ctx: &Context, args: ResolveCommand) {
    let query = sqlx::query!(
        "SELECT id, name FROM subject WHERE name LIKE $1;",
        args.name
    )
    .fetch_all(&ctx.db)
    .await
    .expect("could not resolve subjects");

    if query.is_empty() {
        println!("could not find subject");
        return;
    }

    for record in query.into_iter() {
        println!("{:<22}{:<16}", record.name, record.id);
    }
}
