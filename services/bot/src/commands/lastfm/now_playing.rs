use crate::{Context, Error};
use database::model::{colors::Colors, lastfm::Lastfm};
use poise::serenity_prelude as serenity;
use image::{RgbaImage, imageops, ImageFormat};
use imageproc::{drawing::{draw_filled_rect_mut, draw_text_mut}, rect::Rect};
use ab_glyph::{FontArc, PxScale};
use lazy_static::lazy_static;
use reqwest;
use std::io::Cursor;
use common::utils::truncate_text;

// Design constants
const CANVAS_SIZE: (u32, u32) = (800, 400);
const COVER_SIZE: u32 = 300;
const COVER_POSITION: (i64, i64) = (40, 40);
const TEXT_AREA: (i32, i32) = (370, 80);
const TEXT_COLOR: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const BACKGROUND_COLOR: image::Rgba<u8> = image::Rgba([30, 30, 30, 255]);
const TEXT_BG_COLOR: image::Rgba<u8> = image::Rgba([0, 0, 0, 180]);

lazy_static! {
    static ref FONT: FontArc = {
        let font_data = include_bytes!("../../../../../assets/fonts/Arial.ttf");
        FontArc::try_from_slice(font_data).expect("Error loading font")
    };
}

#[poise::command(
    slash_command,
    prefix_command,
    aliases("np", "now"),
    description_localized("en-US", "Display your current song from Last.fm")
)]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let user_id = ctx.author().id.get();

    // Get Last.fm data
    let session = Lastfm::get(&data.db, user_id)
        .await?
        .ok_or("Link your account with /login")?;

    let (track_opt, _user) = tokio::try_join!(
        data.lastfm.get_current_track(session),
        data.lastfm.get_user_info(user_id)
    )?;

    let track = track_opt.ok_or("No music currently playing")?;

    // Get image URLs
    let (small_url, large_url) = data.lastfm.get_image_urls(&track.image)?;

    let image_bytes = generate_image(
        &track.name,
        &track.artist.text,
        track.album.as_ref().map(|a| a.text.as_str()).unwrap_or("Unknown"),
        large_url
    ).await?;

    // Create embed
    let embed = create_embed(ctx, &track, small_url).await?;

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .attachment(serenity::CreateAttachment::bytes(image_bytes, "now_playing.png"))
    )
        .await?;

    Ok(())
}

async fn generate_image(
    track_name: &str,
    artist_name: &str,
    album_name: &str,
    cover_url: &str
) -> Result<Vec<u8>, Error> {
    // Clone the strings to avoid lifetime issues
    let track_name = track_name.to_string();
    let artist_name = artist_name.to_string();
    let album_name = album_name.to_string();
    let cover_url = cover_url.to_string();

    // Fetch image data
    let bytes = reqwest::get(&cover_url).await?.bytes().await?;

    let result = tokio::task::spawn_blocking(move || {
        let cover = image::load_from_memory(&bytes)?
            .resize_exact(COVER_SIZE, COVER_SIZE, imageops::FilterType::CatmullRom);

        let mut canvas = RgbaImage::from_pixel(CANVAS_SIZE.0, CANVAS_SIZE.1, BACKGROUND_COLOR);

        imageops::overlay(&mut canvas, &cover, COVER_POSITION.0, COVER_POSITION.1);

        // Draw text background
        draw_filled_rect_mut(
            &mut canvas,
            Rect::at(TEXT_AREA.0 - 20, TEXT_AREA.1 - 20).of_size(400, 240),
            TEXT_BG_COLOR
        );

        // Draw track information
        draw_text_mut(
            &mut canvas,
            TEXT_COLOR,
            TEXT_AREA.0,
            100,
            PxScale::from(36.0),
            &*FONT,
            &truncate_text(&track_name, 30)
        );

        draw_text_mut(
            &mut canvas,
            TEXT_COLOR,
            TEXT_AREA.0,
            160,
            PxScale::from(24.0),
            &*FONT,
            &format!("Artist: {}", artist_name)
        );

        draw_text_mut(
            &mut canvas,
            TEXT_COLOR,
            TEXT_AREA.0,
            200,
            PxScale::from(24.0),
            &*FONT,
            &format!("Album: {}", album_name)
        );

        let mut buffer = Cursor::new(Vec::new());
        canvas.write_to(&mut buffer, ImageFormat::Png)?;
        Ok::<_, Error>(buffer.into_inner())
    }).await??;

    Ok(result)
}

async fn create_embed(ctx: Context<'_>, track: &lastfm::Track, image_url: &str) -> Result<serenity::CreateEmbed, Error> {
    let color = Colors::get(&ctx.data().db.cache, ctx.data().http_client.clone(), image_url)
        .await?
        .map(|c| serenity::Colour::from_rgb(c[0], c[1], c[2]))
        .unwrap_or(serenity::Colour::DARK_GREY);

    Ok(serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new("Now Playing")
            .icon_url(ctx.author().face()))
        .title(truncate_text(&track.name, 256))
        .url(&track.url)
        .description(format!(
            "**{}**\n*{}*",
            track.artist.text,
            track.album.as_ref()
                .map(|a| a.text.as_str())
                .unwrap_or("Unknown album")
        ))
        .color(color)
        .image("attachment://now_playing.png"))
}