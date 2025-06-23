use anyhow::{Error, Result};
use chrono::Utc;
use common::config::CONFIG;
use database::model::lastfm::Lastfm;
use std::sync::Arc;

pub mod lastfm;
pub use lastfm::*;

pub struct LastFmClient {
    api_key: String,
    api_secret: String,
    redirect_uri: String,
    db: Arc<database::DatabaseHandler>,
    http_client: reqwest::Client,
}
impl LastFmClient {
    pub async fn new(
        http_client: reqwest::Client,
        database_handler: Arc<database::DatabaseHandler>,
    ) -> Result<Self> {
        Ok(Self {
            api_key: CONFIG.authentication.lastfm_key.clone(),
            api_secret: CONFIG.authentication.lastfm_secret.clone(),
            redirect_uri: CONFIG.authentication.lastfm_redirect_uri.clone(),
            db: database_handler,
            http_client,
        })
    }

    pub fn generate_auth_url(&self, discord_user_id: &str) -> String {
        format!(
            "https://www.last.fm/api/auth/?api_key={}&cb={}/{}",
            self.api_key, self.redirect_uri, discord_user_id
        )
    }

    pub async fn handle_callback(&self, token: &str, user_id: u64) -> Result<()> {
        let session = self.get_session(token).await?;
        let new_lastfm = Lastfm {
            user_id: user_id as i64,
            lastfm_username: session.name.clone(),
            session_key: session.key.clone(),
            token: token.to_string(),
        };
        new_lastfm
            .set(
                &self.db,
                user_id,
                session.name,
                session.key,
                token.to_string(),
            )
            .await?;
        Ok(())
    }

    async fn get_session(&self, token: &str) -> std::result::Result<LastFmSession, Error> {
        let params = [
            ("method", "auth.getSession"),
            ("api_key", &self.api_key),
            ("token", token),
        ];
        let signature = self.generate_signature(&params);
        let url = "https://ws.audioscrobbler.com/2.0/";
        let response_text = self
            .http_client
            .get(url)
            .query(&params)
            .query(&[("api_sig", signature), ("format", "json".to_owned())])
            .send()
            .await?
            .text()
            .await?;
        let response: LastFmSessionResponse = serde_json::from_str(&response_text)?;
        if let Some(error_code) = response.error {
            return Err(anyhow::anyhow!(
                "Last.fm API error {}: {}",
                error_code,
                response.message.unwrap_or_default()
            ));
        }
        response
            .session
            .ok_or_else(|| anyhow::anyhow!("Missing session data in Last.fm response"))
    }

    fn generate_signature(&self, params: &[(&str, &str)]) -> String {
        let mut params = params.to_vec();
        params.sort_by_key(|&(k, _)| k);
        let mut param_string = String::new();
        for (key, value) in &params {
            param_string.push_str(key);
            param_string.push_str(value);
        }
        param_string.push_str(&self.api_secret);
        format!("{:x}", md5::compute(param_string))
    }

    /// Gets the current (or paused) track of the user.
    /// Uses the "user.getRecentTracks" endpoint with limit = 2 and applies the following logic:
    /// - If a track with the attribute nowplaying=="true" is found, that track is returned.
    /// - If not, the timestamp of the first track is compared to the current time; if the difference is less than a threshold (e.g., 90 seconds),
    ///   it is assumed that it is the track being played (or paused) and is returned.
    /// - Otherwise, the first available track is returned.
    pub async fn get_current_track(&self, session: Lastfm) -> Result<Option<Track>> {
        let params = [
            ("method", "user.getRecentTracks"),
            ("user", &session.lastfm_username),
            ("api_key", &self.api_key),
            ("limit", "2"),
            ("format", "json"),
        ];
        let response_text = self
            .http_client
            .get("https://ws.audioscrobbler.com/2.0/")
            .query(&params)
            .send()
            .await?
            .text()
            .await?;
        let response: LastFmRecentTracksResponse = serde_json::from_str(&response_text)?;
        let tracks = response.recenttracks.track;
        if let Some(first) = tracks.first() {
            if first.attr.as_ref().and_then(|a| a.nowplaying.as_deref()) == Some("true") {
                return Ok(Some(first.clone()));
            }
            if let Some(date) = &first.date {
                if let Ok(track_time) = date.uts.parse::<i64>() {
                    let now = Utc::now().timestamp();
                    if now - track_time < 90 {
                        return Ok(Some(first.clone()));
                    }
                }
            }
        }
        Ok(tracks.into_iter().next())
    }

    pub async fn get_track_info(
        &self,
        user_id: u64,
        artist: &str,
        track_name: &str,
    ) -> Result<TrackInfo> {
        let session = self.get_user_session(user_id).await?;
        let params = [
            ("method", "track.getInfo"),
            ("artist", artist),
            ("track", track_name),
            ("username", &session.lastfm_username),
            ("api_key", &self.api_key),
            ("format", "json"),
        ];
        let response = self
            .http_client
            .get("https://ws.audioscrobbler.com/2.0/")
            .query(&params)
            .send()
            .await?
            .json::<LastFmTrackInfoResponse>()
            .await?;
        Ok(response.track)
    }

    pub async fn get_user_info(&self, user_id: u64) -> Result<UserInfo> {
        let session = self.get_user_session(user_id).await?;
        let params = [
            ("method", "user.getInfo"),
            ("user", &session.lastfm_username),
            ("api_key", &self.api_key),
            ("format", "json"),
        ];
        let response = self
            .http_client
            .get("https://ws.audioscrobbler.com/2.0/")
            .query(&params)
            .send()
            .await?
            .json::<LastFmUserInfoResponse>()
            .await?;

        Ok(response.user)
    }

    pub async fn get_user_session(&self, user_id: u64) -> Result<Lastfm> {
        match self.db.cache.get_session(user_id).await? {
            Some(session) => Ok(session),
            None => Err(anyhow::anyhow!("No session found")),
        }
    }

    /// Get the small and large image URLs from Lastfm.
    pub fn get_image_urls<'a>(
        &self,
        images: &'a [Image],
    ) -> Result<(&'a str, &'a str, &'a str), Error> {
        let small = images
            .iter()
            .find(|i| i.size == ImageSizes::Small)
            .ok_or_else(|| anyhow::anyhow!("Small URL not found"))?;

        let large = images
            .iter()
            .find(|i| i.size == ImageSizes::Large)
            .ok_or_else(|| anyhow::anyhow!("Large URL not found"))?;

        let extra_large = images
            .iter()
            .find(|i| i.size == ImageSizes::ExtraLarge)
            .ok_or_else(|| anyhow::anyhow!("Large URL not found"))?;

        Ok((&small.text, &large.text, &extra_large.text))
    }

    pub async fn get_weekly_track_chart(&self, username: &str) -> Result<WeeklyTrackChart> {
        let params = [
            ("method", "user.getWeeklyTrackChart"),
            ("user", username),
            ("api_key", &self.api_key),
            ("format", "json"),
        ];

        let response = self
            .http_client
            .get("https://ws.audioscrobbler.com/2.0/")
            .query(&params)
            .send()
            .await?
            .json::<WeeklyTrackChartResponse>()
            .await?;

        Ok(response.weekly_track_chart)
    }

    pub async fn get_track_play_counts(
        &self,
        user_id: u64,
        artist: &str,
        track_name: &str,
    ) -> Result<(usize, usize)> {
        let session = self.get_user_session(user_id).await?;

        // Get weekly track chart
        let weekly_chart = self
            .get_weekly_track_chart(&session.lastfm_username)
            .await?;

        // Find the track in the weekly chart
        let mut weekly = 0;
        for track in &weekly_chart.track {
            if track.artist.name.eq_ignore_ascii_case(artist)
                && track.name.eq_ignore_ascii_case(track_name)
            {
                weekly = track.playcount.parse::<usize>().unwrap_or(0);
                break;
            }
        }

        // For monthly, we'll use the track info which includes total plays
        // and estimate monthly as 1/4 of total plays, but at least the weekly count
        let track_info = self.get_track_info(user_id, artist, track_name).await?;
        let total_plays = track_info.userplaycount.parse::<usize>().unwrap_or(0);

        // Estimate monthly plays as max of weekly count or 1/4 of total plays
        // This is a reasonable approximation without having to fetch a month of data
        let monthly = std::cmp::max(weekly, total_plays / 4);

        Ok((weekly, monthly))
    }
}
