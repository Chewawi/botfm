pub mod image;
pub mod font;

use tracing_subscriber::fmt::time::UtcTime;
use tracing::info;

use time::format_description;

/// Initialises the logging system.
pub fn tracing_init() {
    let description = "[year]-[month]-[day] [hour]:[minute]:[second]";

    tracing_subscriber::fmt()
        .with_timer(UtcTime::new(format_description::parse(description).unwrap()))
        .with_line_number(true)
        .init();

    info!("Initialised logging");
}

/// Truncates a string to a maximum length.
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() > max_length {
        format!("{}...", &text[..max_length-3])
    } else {
        text.to_string()
    }
}
