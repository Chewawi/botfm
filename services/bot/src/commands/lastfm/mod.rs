use crate::Data;

pub mod login;
pub mod now_playing;
pub mod track_plays;

pub fn register(
) -> Vec<poise::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![
        now_playing::now_playing(),
        track_plays::track_plays(),
        login::login(),
    ]
}
