// use crate::core::structs::Error;
// use ab_glyph::PxScale;
// use common::utils::{font, truncate_text};
// use image::imageops::FilterType;
// use image::{imageops, DynamicImage, GenericImage, ImageFormat, Rgba, RgbaImage};
// use imageproc::drawing::draw_text_mut;
// use std::io::Cursor;
// 
// // Layout Constants
// const CANVAS_SIZE: (u32, u32) = (350, 140);
// const BLUR_RADIUS_BACKGROUND: f32 = 30.0;
// 
// const EDGE_PADDING: u32 = 10;
// const INNER_PADDING: u32 = 15;
// const TEXT_LINE_HEIGHT: i32 = 30;
// const TEXT_FONT_SCALE: f32 = 22.0;
// const TEXT_COLOR: Rgba<u8> = Rgba([230, 230, 230, 255]);
// const MAX_TEXT_CHARS: usize = 28;
// 
// const ALBUM_NAME_X: i32 = EDGE_PADDING as i32;
// const ALBUM_NAME_Y: i32 = EDGE_PADDING as i32;
// 
// const COVER_W: u32 = 85;
// const COVER_H: u32 = 85;
// const COVER_X: i64 = EDGE_PADDING as i64;
// const COVER_Y: i64 = (EDGE_PADDING as i32 + TEXT_LINE_HEIGHT) as i64;
// 
// const INFO_TEXT_X: i32 = (EDGE_PADDING + COVER_W + INNER_PADDING) as i32;
// const INFO_TEXT_Y_START: i32 = COVER_Y as i32 + INNER_PADDING as i32;
// 
// pub async fn now_playing_card(
//     track_name_ref: &str,
//     artist_name_ref: &str,
//     album_name_ref: &str,
//     cover_url: &str,
// ) -> Result<Vec<u8>, Error> {
//     let bytes = reqwest::get(cover_url).await?.bytes().await?;
// 
//     let track_name = track_name_ref.to_string();
//     let artist_name = artist_name_ref.to_string();
//     let album_name = album_name_ref.to_string();
// 
//     let result = tokio::task::spawn_blocking(move || {
//         let cover_dynamic: DynamicImage = image::load_from_memory(&bytes)?;
// 
//         // Create a high-quality background from cover
//         // Use Lanczos3 filter for higher quality resizing (sharper than Gaussian)
//         let enlarged_cover_bg =
//             cover_dynamic.resize_exact(CANVAS_SIZE.0 * 2, CANVAS_SIZE.1 * 2, FilterType::Lanczos3);
// 
//         // Apply blur with adjusted radius for a smoother effect
//         let blurred_background =
//             imageops::blur(&enlarged_cover_bg.to_rgba8(), BLUR_RADIUS_BACKGROUND);
// 
//         // Resize back down to target size with high-quality filter to avoid pixelation
//         let mut canvas = RgbaImage::new(CANVAS_SIZE.0, CANVAS_SIZE.1);
//         canvas.copy_from(
//             &imageops::resize(
//                 &blurred_background,
//                 CANVAS_SIZE.0,
//                 CANVAS_SIZE.1,
//                 FilterType::Lanczos3,
//             ),
//             0,
//             0,
//         )?;
// 
//         // Add the main cover image (sharp) with high-quality resizing
//         let main_cover_resized = cover_dynamic.resize_exact(COVER_W, COVER_H, FilterType::Lanczos3);
//         imageops::overlay(
//             &mut canvas,
//             &main_cover_resized.to_rgba8(),
//             COVER_X,
//             COVER_Y,
//         );
// 
//         // Prepare text content
//         let display_album_name = truncate_text(&album_name, MAX_TEXT_CHARS);
//         let display_track_name = truncate_text(&track_name, MAX_TEXT_CHARS);
//         let display_artist_name = truncate_text(&artist_name, MAX_TEXT_CHARS);
// 
//         // Draw text elements
//         draw_text_mut(
//             &mut canvas,
//             TEXT_COLOR,
//             ALBUM_NAME_X,
//             ALBUM_NAME_Y,
//             PxScale::from(TEXT_FONT_SCALE - 2.0),
//             &*font::SPOTIFY_BOLD,
//             &display_album_name,
//         );
// 
//         draw_text_mut(
//             &mut canvas,
//             TEXT_COLOR,
//             INFO_TEXT_X,
//             INFO_TEXT_Y_START,
//             PxScale::from(TEXT_FONT_SCALE),
//             &*font::SPOTIFY_BOLD,
//             &display_track_name,
//         );
// 
//         draw_text_mut(
//             &mut canvas,
//             TEXT_COLOR,
//             INFO_TEXT_X,
//             INFO_TEXT_Y_START + TEXT_LINE_HEIGHT,
//             PxScale::from(TEXT_FONT_SCALE),
//             &*font::SPOTIFY_REGULAR,
//             &display_artist_name,
//         );
// 
//         // Encode final image
//         let mut buffer = Cursor::new(Vec::new());
//         canvas.write_to(&mut buffer, ImageFormat::Png)?;
//         Ok::<_, Error>(buffer.into_inner())
//     })
//     .await??;
// 
//     Ok(result)
// }
