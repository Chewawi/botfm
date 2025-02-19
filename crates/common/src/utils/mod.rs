pub mod image;

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
