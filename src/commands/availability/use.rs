use crate::{Availability, Context};
use clap::Args;
use souvenir::Id;

#[derive(Debug, Args)]
pub struct UseCommand {
    id: String,
}

pub async fn evaluate(ctx: &Context, args: UseCommand) {
    let id = Id::<Availability>::parse(&args.id).unwrap().as_i64();

    sqlx::query!("UPDATE parameters SET availability = $1;", id)
        .execute(&ctx.db)
        .await
        .expect("could not set availability");
}
