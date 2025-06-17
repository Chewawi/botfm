use std::vec;
use crate::core::structs::{Context, Error};
use common::utils::truncate_text;
use database::model::{colors::Colors, lastfm::Lastfm};
use lumi::serenity_prelude as serenity;
use lumi::CreateReply;

#[lumi::command(
    slash_command,
    prefix_command,
    aliases("np", "now"),
    description_localized("en-US", "Display your current song from Last.fm")
)]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), Error> {
    let _ = ctx.defer_or_broadcast().await;
    let data = ctx.data();
    let user_id = ctx.author().id.get();

    let session_result = Lastfm::get(&data.db, user_id).await;

    let session = match session_result {
        Ok(Some(session)) => session,
        Ok(None) => return Err("Link your account with /login".into()),
        Err(e) => return Err(Error::from(e)),
    };

    let track_future = data.lastfm.get_current_track(session.clone());
    let user_future = data.lastfm.get_user_info(user_id);

    let (track_opt, user) = tokio::try_join!(track_future, user_future)?;

    let track = track_opt.ok_or("No music currently playing")?;

    let (small_url, _, medium_url) = data.lastfm.get_image_urls(&track.image)?;

    let track_info_future = data.lastfm.get_track_info(
        user_id, 
        &track.artist.text, 
        &track.name
    );

    let color_future = Colors::get(
        &data.db.cache,
        data.http_client.clone(),
        small_url
    );

    // Handle potential errors from the futures
    let (track_info, color_result) = tokio::try_join!(track_info_future, color_future).unwrap_or_else(|err| {
        // Return default values
        (
            lastfm::TrackInfo {
                playcount: "0".to_string(),
                userplaycount: "0".to_string()
            },
            None
        )
    });

    let container_future = async {
        let color = color_result
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

        serenity::CreateContainer::new(vec![
            serenity::CreateComponent::Section(
                serenity::CreateSection::new(
                    vec![serenity::CreateSectionComponent::TextDisplay(serenity::CreateTextDisplay::new(text_display_content))],
                    serenity::CreateSectionAccessory::Thumbnail(
                        serenity::CreateThumbnail::new(serenity::CreateUnfurledMediaItem::new(medium_url))
                    ),
                )
            ),
            serenity::CreateComponent::Separator(serenity::CreateSeparator::new(true).spacing(serenity::Spacing::Small)),
            serenity::CreateComponent::TextDisplay(serenity::CreateTextDisplay::new(format!(
                "-# plays: `{}` | scrobbles: `{}`",
                track_info.userplaycount, user.playcount
            ))),
        ])
        .accent_color(color.0)
    };

    let container = container_future.await;

    ctx.send(
        CreateReply::default()
            .flags(serenity::MessageFlags::IS_COMPONENTS_V2)
            .components(&[serenity::CreateComponent::Container(container)])
            .reply(true),
    )
    .await?;

    Ok(())
}
