mod handlers;

use lumi::serenity_prelude::{self as serenity, FullEvent};
use crate::core::structs::{Data, Error};

use handlers::*;

pub struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, ctx: &serenity::Context, event: &serenity::FullEvent) {
        let _ = event_handler(ctx, event).await;
    }
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    let data = ctx.data::<Data>();

    match event {
        FullEvent::Ready { data_about_bot } => {
            ready::ready(ctx, data_about_bot, data).await?;
        }
        _ => {}
    }
    Ok(())
}