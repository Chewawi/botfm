use crate::core::structs::{Context, Error};
use database::model::colors::Colors;
use database::model::lastfm::Lastfm;
use lumi::serenity_prelude as serenity;
use serenity::all::MessageFlags;
use tokio::join;

#[lumi::command(slash_command, prefix_command, aliases("tp"))]
pub async fn track_plays(ctx: Context<'_>) -> Result<(), Error> {
    let _ = ctx.defer_or_broadcast().await;

    let data = ctx.data();
    let user_id = ctx.author().id.get();

    // Get the user's Last.fm session from the database
    let lastfm_user = match Lastfm::get(&data.db, user_id).await {
        Ok(Some(user)) => user,
        _ => {
            ctx.say("You haven't linked your Last.fm account yet. Use the `/login` command.")
                .await?;
            return Ok(());
        }
    };

    // Fetch the currently playing track (or the most recent one)
    let track = match data.lastfm.get_current_track(lastfm_user.clone()).await {
        Ok(Some(track)) => track,
        Ok(None) => {
            ctx.say("You're not playing anything right now.").await?;
            return Ok(());
        }
        Err(err) => {
            ctx.say(format!("Error fetching current track: {}", err))
                .await?;
            return Ok(());
        }
    };

    let track_name = track.name.clone();
    let artist_name = track.artist.text.clone();

    // Extract image URLs
    let (small_url, large_url, _) = data.lastfm.get_image_urls(&track.image)?;

    // Start all async operations concurrently
    let track_info_future = data.lastfm.get_track_info(user_id, &artist_name, &track_name);
    let play_counts_future = data.lastfm.get_track_play_counts(user_id, &artist_name, &track_name);
    let image_color_future = Colors::get(&data.db.cache, data.http_client.clone(), small_url);

    // Wait for all operations to complete
    let (track_info_result, play_counts_result, image_color_result) = join!(
        track_info_future,
        play_counts_future,
        image_color_future
    );

    // Handle track info result
    let track_info = match track_info_result {
        Ok(info) => info,
        Err(err) => {
            ctx.say(format!("Error fetching track info: {}", err))
                .await?;
            return Ok(());
        }
    };

    // Handle play counts result
    let (weekly, monthly) = match play_counts_result {
        Ok(counts) => counts,
        Err(err) => {
            ctx.say(format!("Error fetching play stats: {}", err))
                .await?;
            return Ok(());
        }
    };

    // Extract image URLs and compute accent color
    let image_color = image_color_result
        .unwrap_or(None)
        .map(|c| serenity::Colour::from_rgb(c[0], c[1], c[2]))
        .unwrap_or(serenity::Colour::DARK_GREY);

    // Build Discord container
    let container = serenity::CreateContainer::new(vec![
        serenity::CreateComponent::Section(serenity::CreateSection::new(
            vec![serenity::CreateSectionComponent::TextDisplay(
                serenity::CreateTextDisplay::new(format!(
                    "**{}** total plays for **{}** by **{}**",
                    track_info.userplaycount, track.name, track.artist.text
                )),
            )],
            serenity::CreateSectionAccessory::Thumbnail(serenity::CreateThumbnail::new(
                serenity::CreateUnfurledMediaItem::new(large_url),
            )),
        )),
        serenity::CreateComponent::Separator(
            serenity::CreateSeparator::new(true).spacing(serenity::Spacing::Small),
        ),
        serenity::CreateComponent::TextDisplay(serenity::CreateTextDisplay::new(format!(
            "-# `{}` plays last week — `{}` plays last month",
            weekly, monthly
        ))),
    ])
    .accent_color(image_color.0);

    // Send the message
    ctx.send(
        lumi::CreateReply::default()
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(&[serenity::CreateComponent::Container(container)])
            .reply(true),
    )
    .await?;

    Ok(())
}
