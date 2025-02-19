use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;
use anyhow::Result;
use reqwest::Client;

pub async fn get_image_color(
    http_client: Client,
    image_url: impl Into<String>,
) -> Result<Vec<u8>> {
    let response = http_client
        .get(image_url.into())
        .send()
        .await?
        .bytes()
        .await?;
    let img = image::load_from_memory(&response)?;
    let dominant_color = get_dominant_color(&img)?;
    Ok(dominant_color)
}

pub fn get_dominant_color(img: &DynamicImage) -> Result<Vec<u8>> {
    let rgba = img.to_rgba8();
    let pixels = rgba.as_raw();
    let total_pixels = (rgba.width() * rgba.height()) as u32;

    let (sum_r, sum_g, sum_b) = pixels
        .par_chunks(4)
        .map(|p| (p[0] as u32, p[1] as u32, p[2] as u32))
        .reduce(|| (0, 0, 0), |(r1, g1, b1), (r2, g2, b2)| {
            (r1 + r2, g1 + g2, b1 + b2)
        });

    let avg_r = (sum_r / total_pixels) as u8;
    let avg_g = (sum_g / total_pixels) as u8;
    let avg_b = (sum_b / total_pixels) as u8;

    Ok(vec![avg_r, avg_g, avg_b])
}
