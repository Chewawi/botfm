use crate::core::structs::{Context, Error};
use common::utils::truncate_text;
use database::model::{colors::Colors, lastfm::Lastfm};
use lumi::serenity_prelude as serenity;
use lumi::CreateReply;
use std::vec;

#[lumi::command(
    slash_command,
    prefix_command,
    aliases("np", "fm", "now"),
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

    // Start all async operations concurrently
    let track_future = data.lastfm.get_current_track(session.clone());
    let user_future = data.lastfm.get_user_info(user_id);

    // Wait for both operations to complete
    let (track_opt, user) = tokio::try_join!(track_future, user_future)?;

    let track = track_opt.ok_or("No music currently playing")?;

    // Get image URLs
    let (small_url, _, medium_url) = data.lastfm.get_image_urls(&track.image)?;

    // Check if the image is the default one
    let is_default_image = small_url == lastfm::DEFAULT_IMAGE_URL;

    // Get track info
    let track_info = data.lastfm.get_track_info(user_id, &track.artist.text, &track.name).await.unwrap_or_else(|_| lastfm::TrackInfo {
        playcount: "0".to_string(),
        userplaycount: "0".to_string(),
    });

    // Only get image color if not the default image
    let color_result = if !is_default_image {
        Colors::get(&data.db.cache, data.http_client.clone(), small_url).await.ok().flatten()
    } else {
        None
    };

    // Map color result to serenity color
    let image_color_opt = color_result.map(|c| serenity::Colour::from_rgb(c[0], c[1], c[2]));

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

    // Build components
    let mut components = vec![
        serenity::CreateComponent::Section(
            serenity::CreateSection::new(
                vec![serenity::CreateSectionComponent::TextDisplay(serenity::CreateTextDisplay::new(text_display_content))],
                serenity::CreateSectionAccessory::Thumbnail(
                    serenity::CreateThumbnail::new(serenity::CreateUnfurledMediaItem::new(medium_url))
                ),
            )
        ),
        serenity::CreateComponent::Separator(
            serenity::CreateSeparator::new(true).spacing(serenity::Spacing::Small)
        ),
        serenity::CreateComponent::TextDisplay(serenity::CreateTextDisplay::new(format!(
            "-# plays: `{}` | scrobbles: `{}`",
            track_info.userplaycount, user.playcount
        ))),
    ];

    // Add "Now Playing" text if track is currently playing
    if let Some(attr) = &track.attr {
        if let Some(nowplaying) = &attr.nowplaying {
            if nowplaying == "true" {
                components.insert(0, serenity::CreateComponent::TextDisplay(
                    serenity::CreateTextDisplay::new("## Now Playing")
                ));
            }
        }
    }

    // Create container
    let mut container = serenity::CreateContainer::new(components);

    // Only set accent color if we have a color
    if let Some(color) = image_color_opt {
        container = container.accent_color(color.0);
    }

    ctx.send(
        CreateReply::default()
            .flags(serenity::MessageFlags::IS_COMPONENTS_V2)
            .components(&[serenity::CreateComponent::Container(container)])
            .reply(true),
    )
        .await?;

    Ok(())
}
