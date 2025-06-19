use crate::core::structs::{Context, Error};
use database::model::colors::Colors;
use database::model::lastfm::Lastfm;
use lumi::serenity_prelude as serenity;
use serenity::all::MessageFlags;

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
    tracing::info!("{} {}", small_url, lastfm::DEFAULT_IMAGE_URL);

    // Check if the image is the default one
    let is_default_image = small_url == lastfm::DEFAULT_IMAGE_URL;

    // Get track info first
    let track_info = match data.lastfm.get_track_info(user_id, &artist_name, &track_name).await {
        Ok(info) => info,
        Err(err) => {
            ctx.say(format!("Error fetching track info: {}", err))
                .await?;
            return Ok(());
        }
    };

    let playcount = track_info.userplaycount.parse::<usize>().unwrap_or(0);

    // Only get play counts if playcount > 0
    let (weekly, monthly) = if playcount > 0 {
        match data.lastfm.get_track_play_counts(user_id, &artist_name, &track_name).await {
            Ok(counts) => counts,
            Err(err) => {
                ctx.say(format!("Error fetching play stats: {}", err))
                    .await?;
                return Ok(());
            }
        }
    } else {
        (0, 0)
    };

    // Only get image color if not the default image
    let image_color_result = if !is_default_image {
        Colors::get(&data.db.cache, data.http_client.clone(), small_url).await.ok().flatten()
    } else {
        None
    };

    // Only map to color if we got a result
    let image_color_opt = image_color_result.map(|c| serenity::Colour::from_rgb(c[0], c[1], c[2]));

    // Build Discord container components
    let mut components = vec![
        serenity::CreateComponent::Section(serenity::CreateSection::new(
            vec![serenity::CreateSectionComponent::TextDisplay(
                serenity::CreateTextDisplay::new(format!(
                    "**{}** total plays for **[{}]({})** by **[{}](https://www.last.fm/music/{})**",
                    playcount, track.name, track.url, track.artist.text, track.artist.text
                )),
            )],
            serenity::CreateSectionAccessory::Thumbnail(serenity::CreateThumbnail::new(
                serenity::CreateUnfurledMediaItem::new(large_url),
            )),
        )),
        serenity::CreateComponent::Separator(
            serenity::CreateSeparator::new(true).spacing(serenity::Spacing::Small),
        ),
    ];

    // Only add weekly/monthly plays if they're not both 0
    if weekly > 0 || monthly > 0 {
        components.push(serenity::CreateComponent::TextDisplay(serenity::CreateTextDisplay::new(format!(
            "-# `{}` plays last week — `{}` plays last month",
            weekly, monthly
        ))));
    }

    // Create container
    let mut container = serenity::CreateContainer::new(components);

    // Only set an accent color if we have a color
    if let Some(color) = image_color_opt {
        container = container.accent_color(color.0);
    }

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
