use crate::Context;
use clap::Args;
use souvenir::Id;

#[derive(Debug, Args)]
pub struct RevertCommand {
    hash: String,
}

pub async fn evaluate(ctx: &Context, args: RevertCommand) {
    if args.hash == "ROOT" {
        sqlx::query!("UPDATE parameters SET schedule = NULL;")
            .execute(&ctx.db)
            .await
            .expect("could not update schedule");

        return;
    }

    let id = Id::parse(&args.hash).unwrap();

    sqlx::query!("UPDATE parameters SET schedule = $1;", id as Id)
        .execute(&ctx.db)
        .await
        .expect("could not update schedule");
}
