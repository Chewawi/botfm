use std::vec;
use crate::core::structs::{Context, Error};
use common::utils::truncate_text;
use database::model::{colors::Colors, lastfm::Lastfm};
use lumi::serenity_prelude as serenity;
use lumi::CreateReply;
use sqlx::types::chrono::Utc;

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

    // Get the user to check when they were last updated
    let user = match data.db.get_user(user_id as i64).await {
        Ok(user) => user,
        Err(e) => return Err(Error::from(e)),
    };

    // Check if the user needs a quick update (more than 3 minutes since last update)
    let needs_update = match user.last_updated {
        Some(last_updated) => {
            let now = Utc::now();
            let minutes_since_update = (now - last_updated).num_minutes();
            minutes_since_update >= 3
        },
        None => true, // No last_updated timestamp means we definitely need an update
    };

    // If the user needs an update, trigger a quick sync
    if needs_update {
        // Use force=false to let the sync_scrobbles method determine the update type
        if let Err(e) = data.lastfm.sync_scrobbles(user_id, false).await {
            // Log the error but continue with potentially stale data
            println!("Warning: Failed to sync scrobbles: {}", e);
        }
    }

    let track_future = data.lastfm.get_current_track(session.clone());
    let user_future = data.lastfm.get_user_info(user_id);

    let (track_opt, user) = tokio::try_join!(track_future, user_future)?;

    let track = track_opt.ok_or("No music currently playing")?;

    // Handle the case where no images are found
    let default_image_url = "https://lastfm.freetls.fastly.net/i/u/64s/2a96cbd8b46e442fc41c2b86b821562f.png";
    let (small_url, medium_url) = match data.lastfm.get_image_urls(&track.image) {
        Ok((small, medium, _)) => (small, medium),
        Err(err) => {
            // Log the error but continue with default values
            println!("Warning: Could not get image URLs: {}", err);
            (default_image_url, default_image_url)
        }
    };

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
    let (track_info, color_result) = match tokio::try_join!(track_info_future, color_future) {
        Ok(result) => result,
        Err(err) => {
            println!("Warning: Error getting track info or color: {}", err);
            // Return default values
            (
                lastfm::TrackInfo { 
                    playcount: "0".to_string(), 
                    userplaycount: "0".to_string() 
                },
                None
            )
        }
    };

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
