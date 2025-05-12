use crate::core::structs::Command;

pub mod login;
pub mod now_playing;
pub mod track_plays;

pub fn register(
) -> Vec<Command> {
    vec![
        now_playing::now_playing(),
        track_plays::track_plays(),
        login::login(),
    ]
}
