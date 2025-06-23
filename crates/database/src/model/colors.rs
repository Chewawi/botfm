use crate::server_cache::ServerCache;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Colors {
    pub image_url: String,
    pub colors: Vec<u8>,
}

impl Colors {
    pub async fn get(
        cache: &ServerCache,
        http: reqwest::Client,
        image_url: &str,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        if let Some(rgba) = cache.get_image_color(image_url).await? {
            return Ok(Some(rgba));
        }

        match common::utils::image::get_image_color(http, image_url).await {
            Ok(image_color) => {
                cache.set_image_color(image_url, &image_color).await?;
                Ok(Some(image_color))
            }
            Err(err) => Err(err),
        }
    }

    pub async fn set(&self, cache: &ServerCache, colors: Vec<u8>) -> anyhow::Result<Option<Self>> {
        cache.set_image_color(&self.image_url, &colors).await?;
        Ok(Some(self.clone()))
    }
}
