use crate::{Context, Error};
use database::model::colors::Colors;
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

    let session = Lastfm::get(&*data.db, user_id)
        .await?
        .ok_or_else(|| Error::from("Missing Last.fm user session"))?;

    let (track_opt, lastfm_user) = tokio::try_join!(
        lastfm_client.get_current_track(session),
        lastfm_client.get_user_info(user_id)
    )?;

    let track = track_opt.ok_or_else(|| Error::from("No current track found"))?;

    let trackinfo = lastfm_client
            .get_track_info(user_id, &track.artist.text, &track.name).await?;

    let title_prefix = if track.attr.as_ref()
        .and_then(|a| a.nowplaying.as_deref()) == Some("true")
    {
        "Now playing"
    } else {
        "Last track"
    };

    let small_url = track
        .image
        .iter()
        .find(|img| img.size == "small")
        .map(|img| &img.text)
        .ok_or_else(|| Error::from("Missing small image URL"))?;
    let large_url = track
        .image
        .iter()
        .find(|img| img.size == "large")
        .map(|img| &img.text)
        .ok_or_else(|| Error::from("Missing large image URL"))?;

    let image_color = Colors::get(&data.db.cache, data.http_client.clone(), small_url)
        .await?
        .ok_or_else(|| Error::from("Could not retrieve image color"))?;

    println!("{:#?}", trackinfo);

    let embed = serenity::CreateEmbed::new()
        .author(
            serenity::CreateEmbedAuthor::new(title_prefix).icon_url(ctx.author().face()),
        )
        .title(&track.name)
        .url(&track.url)
        .description(format!(
            "-# **{}** - *{}*",
            track.artist.text,
            track.album.as_ref().map(|a| a.text.as_str()).unwrap_or("Unknown album")
        ))
        .color(serenity::Colour::from_rgb(
            image_color[0],
            image_color[1],
            image_color[2],
        ))
        .thumbnail(large_url);

    let components = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("play_count")
            .label(format!("{} plays", trackinfo.userplaycount))
            .disabled(true),
        serenity::CreateButton::new("scrobbles")
            .label(format!("{} scrobbles", lastfm_user.playcount))
            .disabled(true),
    ])];

    ctx.send(poise::CreateReply::default().embed(embed).components(components))
        .await?;

    Ok(())
}
