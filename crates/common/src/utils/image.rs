use anyhow::Result;
use image::DynamicImage;
use rayon::prelude::*;
use reqwest::Client;
use tracing;

pub async fn get_image_color(http_client: Client, image_url: impl Into<String>) -> Result<Vec<u8>> {
    // Set a timeout for the HTTP request to prevent long-running requests
    let request = http_client
        .get(image_url.into())
        .timeout(std::time::Duration::from_millis(500)); // 500ms timeout

    // Send the request and handle timeout errors
    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) => {
            if e.is_timeout() {
                // Return a default color on timeout
                return Ok(vec![128, 128, 128]); // Default gray color
            }
            return Err(e.into());
        }
    };

    // Get the bytes from the response
    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(e) => {
            // Return a default color on error
            tracing::warn!("Failed to get image bytes: {}", e);
            return Ok(vec![128, 128, 128]); // Default gray color
        }
    };

    // Load the image and get the dominant color
    match image::load_from_memory(&bytes) {
        Ok(img) => {
            match get_dominant_color(&img) {
                Ok(color) => Ok(color),
                Err(e) => {
                    tracing::warn!("Failed to get dominant color: {}", e);
                    Ok(vec![128, 128, 128]) // Default gray color
                }
            }
        }
        Err(e) => {
            tracing::warn!("Failed to load image: {}", e);
            Ok(vec![128, 128, 128]) // Default gray color
        }
    }
}

pub fn get_dominant_color(img: &DynamicImage) -> Result<Vec<u8>> {
    // Downsample the image to a maximum of 100x100 pixels for faster processing
    let img = if img.width() > 100 || img.height() > 100 {
        let width = std::cmp::min(img.width(), 100);
        let height = std::cmp::min(img.height(), 100);
        img.thumbnail(width, height)
    } else {
        img.clone()
    };

    let rgba = img.to_rgba8();
    let pixels = rgba.as_raw();
    let total_pixels = rgba.width() * rgba.height();

    // Skip processing if there are no pixels
    if total_pixels == 0 {
        return Ok(vec![0, 0, 0]);
    }

    let (sum_r, sum_g, sum_b) = pixels
        .par_chunks(4)
        .map(|p| (p[0] as u32, p[1] as u32, p[2] as u32))
        .reduce(
            || (0, 0, 0),
            |(r1, g1, b1), (r2, g2, b2)| (r1 + r2, g1 + g2, b1 + b2),
        );

    let avg_r = (sum_r / total_pixels) as u8;
    let avg_g = (sum_g / total_pixels) as u8;
    let avg_b = (sum_b / total_pixels) as u8;

    Ok(vec![avg_r, avg_g, avg_b])
}
