use crate::Context;
use clap::Args;
use souvenir::Id;

#[derive(Debug, Args)]
pub struct UseCommand {
    id: String,
}

pub async fn evaluate(ctx: &Context, args: UseCommand) {
    let id = Id::parse(&args.id).unwrap();

    sqlx::query!("UPDATE parameters SET availability = $1;", id as Id)
        .execute(&ctx.db)
        .await
        .expect("could not set availability");
}
