use crate::{Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Initiate Last.fm web authentication login.")
)]
pub async fn login(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    let lastfm_client = data.lastfm.clone();

    let user_id = ctx.author().id.get();
    let auth_url = lastfm_client.generate_auth_url(&user_id.to_string());

    let message = format!("Please visit this URL to authorize: {}", auth_url);
    ctx.say(message).await?;

    Ok(())
}
