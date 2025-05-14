use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct LastFmSessionResponse {
    #[serde(default)]
    pub session: Option<LastFmSession>,
    #[serde(default)]
    pub error: Option<i32>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LastFmSession {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct LastFmRecentTracksResponse {
    pub recenttracks: RecentTracks,
}

#[derive(Debug, Deserialize)]
pub struct RecentTracks {
    #[serde(alias = "track", deserialize_with = "deserialize_track")]
    pub track: Vec<Track>,
}

// chatgpt :p
fn deserialize_track<'de, D>(deserializer: D) -> Result<Vec<Track>, D::Error>
where
    D: Deserializer<'de>,
{
    struct TrackVisitor;
    impl<'de> Visitor<'de> for TrackVisitor {
        type Value = Vec<Track>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("erm what the sigma")
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Vec<Track>, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut tracks = Vec::new();
            while let Some(track) = seq.next_element()? {
                tracks.push(track);
            }
            Ok(tracks)
        }
        fn visit_map<M>(self, map: M) -> Result<Vec<Track>, M::Error>
        where
            M: MapAccess<'de>,
        {
            let track = Track::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(vec![track])
        }
    }
    deserializer.deserialize_any(TrackVisitor)
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrackArtist {
    #[serde(rename = "#text")]
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    pub name: String,
    pub artist: Artist,
    #[serde(rename = "@attr")]
    pub attr: Option<TrackAttr>,
    pub mbid: String,
    pub album: Option<Album>,
    pub image: Vec<Image>,
    pub streamable: String,
    pub url: String,
    pub date: Option<Date>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Album {
    pub mbid: String,
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageSizes {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub size: ImageSizes,
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Artist {
    pub mbid: String,
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrackAttr {
    pub nowplaying: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Date {
    pub uts: String,
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct LastFmTrackInfoResponse {
    pub track: TrackInfo,
}

#[derive(Debug, Deserialize)]
pub struct TrackInfo {
    pub playcount: String,
    pub userplaycount: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub realname: String,
    pub playcount: String,
    pub artist_count: String,
    pub album_count: String,
    pub country: String,
    pub url: String,
    pub image: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct LastFmUserInfoResponse {
    pub user: UserInfo,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyChartListResponse {
    #[serde(rename = "weeklychartlist")]
    pub weekly_chart_list: WeeklyChartList,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyChartList {
    #[serde(rename = "chart")]
    pub charts: Vec<ChartRange>,
}

#[derive(Debug, Deserialize)]
pub struct ChartRange {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyTrackChartResponse {
    #[serde(rename = "weeklytrackchart")]
    pub weekly_track_chart: WeeklyTrackChart,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyTrackChart {
    #[serde(default)]
    pub track: Vec<WeeklyTrack>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeeklyTrack {
    pub name: String,
    pub playcount: String,
    pub artist: TrackArtist,
}
