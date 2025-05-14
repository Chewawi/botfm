use crate::core::structs::{Context, Error};
use crate::images::lastfm::now_playing_card::now_playing_card;
use common::utils::truncate_text;
use database::model::{colors::Colors, lastfm::Lastfm};
use lumi::serenity_prelude as serenity;
use lumi::CreateReply;
use ::serenity::builder::{
    CreateAttachment, CreateComponent, CreateContainer, CreateMediaGallery, CreateMediaGalleryItem,
    CreateSeparator, CreateTextDisplay, CreateUnfurledMediaItem, Spacing,
};

#[lumi::command(
    slash_command,
    prefix_command,
    aliases("np", "now"),
    description_localized("en-US", "Display your current song from Last.fm")
)]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let user_id = ctx.author().id.get();

    let session = Lastfm::get(&data.db, user_id)
        .await?
        .ok_or("Link your account with /login")?;

    let (track_opt, user) = tokio::try_join!(
        data.lastfm.get_current_track(session),
        data.lastfm.get_user_info(user_id)
    )?;

    let track = track_opt.ok_or("No music currently playing")?;
    let (small_url, _, extra_url) = data.lastfm.get_image_urls(&track.image)?;

    let track_info = data
        .lastfm
        .get_track_info(user_id, &track.artist.text, &track.name)
        .await?;

    let card_bytes = now_playing_card(
        &track.name,
        &track.artist.text,
        track
            .album
            .as_ref()
            .map(|a| a.text.as_str())
            .unwrap_or("Unknown Album"),
        extra_url,
    )
    .await?;

    let container = create_container(ctx, &track, &track_info, &user, small_url).await?;

    ctx.send(
        CreateReply::default()
            .flags(serenity::MessageFlags::IS_COMPONENTS_V2)
            .components(&[CreateComponent::Container(container)])
            .attachment(CreateAttachment::bytes(card_bytes, "now_playing.png"))
            .reply(true),
    )
    .await?;

    Ok(())
}

async fn create_container<'a>(
    ctx: Context<'_>,
    track: &'a lastfm::Track,
    track_info: &'a lastfm::TrackInfo,
    user: &'a lastfm::UserInfo,
    image_url: &str,
) -> Result<CreateContainer<'a>, Error> {
    let color = Colors::get(
        &ctx.data().db.cache,
        ctx.data().http_client.clone(),
        image_url,
    )
    .await?
    .map(|c| serenity::Colour::from_rgb(c[0], c[1], c[2]))
    .unwrap_or(serenity::Colour::DARK_GREY);

    let text_display_content = format!(
        "**[{}]({})**\n-# {} - {}",
        truncate_text(&track.name, 40),
        track.url,
        truncate_text(&track.artist.text, 30),
        track
            .album
            .as_ref()
            .map(|a| truncate_text(&a.text, 50))
            .unwrap_or_else(|| "Unknown Album".to_string()),
    );

    Ok(CreateContainer::new(vec![
        CreateComponent::TextDisplay(CreateTextDisplay::new(text_display_content)),
        CreateComponent::MediaGallery(CreateMediaGallery::new(vec![CreateMediaGalleryItem::new(
            CreateUnfurledMediaItem::new("attachment://now_playing.png"),
        )])),
        CreateComponent::Separator(CreateSeparator::new(true).spacing(Spacing::Small)),
        CreateComponent::TextDisplay(CreateTextDisplay::new(format!(
            "-# plays: `{}` | scrobbles: `{}`",
            track_info.userplaycount, user.playcount
        ))),
    ])
    .accent_color(color.0))
}
