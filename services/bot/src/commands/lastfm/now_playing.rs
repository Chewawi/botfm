use crate::utils::image::get_image_color;
use crate::{Context, Error};
use database::model::lastfm::Lastfm;
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    prefix_command,
    aliases("np", "now"),
    description_localized("en-US", "Shows the currently playing song of a Last.fm user.")
)]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let lastfm_client = data.lastfm.clone();
    let user_id = ctx.author().id.get();

    let db_clone = data.db.clone();
    let db = Lastfm::get(&db_clone as &database::DatabaseHandler, user_id);

    let lastfm_user = match db.await {
        Ok(user) => user,
        Err(_) => {
            ctx.say("You haven't linked your Last.fm account yet. Use the `/login` command.")
                .await?;
            return Ok(());
        }
    };

    match lastfm_client
        .get_current_track(lastfm_user.expect("Missing Last.fm user"))
        .await
    {
        Ok(track_opt) => {
            let lastfm_user = lastfm_client.get_user_info(user_id).await?;

            let track = track_opt.unwrap();

            let title_prefix =
                if track.attr.as_ref().and_then(|a| a.nowplaying.as_deref()) == Some("true") {
                    "Now playing"
                } else {
                    "Last track"
                };

            let image_url = &track
                .image
                .iter()
                .find(|image| image.size == "large")
                .unwrap()
                .text;

            let image_color = get_image_color(data.http_client.clone(), image_url.clone()).await?;

            let embed = serenity::CreateEmbed::new()
                .author(
                    serenity::CreateEmbedAuthor::new(title_prefix).icon_url(ctx.author().face()),
                )
                .title(track.name)
                .url(track.url)
                .description(format!(
                    "-# **{}** - *{}*",
                    track.artist.text,
                    track.album.unwrap().text
                ))
                .color(serenity::Colour::from_rgb(
                    image_color[0],
                    image_color[1],
                    image_color[2],
                ))
                .thumbnail(image_url.clone());

            let components = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("play_count")
                    .label(format!("{} total scrobbles", lastfm_user.playcount))
                    .disabled(true),
            ])];

            ctx.send(
                poise::CreateReply::default()
                    .embed(embed)
                    .components(components),
            )
            .await?;
        }
        Err(err) => {
            ctx.say(format!("Error fetching current track: {}", err))
                .await?;
        }
    }

    Ok(())
}
