use crate::commands::register_all_commands;
use crate::core::structs::{Data, Error};
use lumi::builtins::create_application_commands;
use lumi::serenity_prelude::{self as serenity, small_fixed_array, Ready};
use serenity::all::Command as RawCommand;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tracing::info;

pub async fn ready(ctx: &serenity::Context, ready: &Ready, data: Arc<Data>) -> Result<(), Error> {
    let activity_data = serenity::ActivityData {
        name: small_fixed_array::FixedString::from_str_trunc("Milo J"),
        kind: serenity::ActivityType::Listening,
        state: None,
        url: None,
    };
    ctx.set_activity(Some(activity_data));

    let commands = create_application_commands(&*register_all_commands());

    if !data.has_started.swap(true, Ordering::SeqCst) {
        RawCommand::set_global_commands(&ctx.http, &*commands).await?;
        info!("Global commands set!");

        info!(
            "Bot is ready! Logged in as {} (ID: {})",
            ready.user.name, ready.user.id
        );
    }

    Ok(())
}
