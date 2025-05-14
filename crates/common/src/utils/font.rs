use ab_glyph::FontArc;
use lazy_static::lazy_static;

const FONTS_DIR: &str = "../../../../assets/fonts";

lazy_static! {
    //pub static ref ARIAL: FontArc = {
    //    let font_data = include_bytes!("{}/Arial.ttf".format(FONTS_DIR));
    //    FontArc::try_from_slice(font_data).expect("Error loading font")
    //};
    pub static ref SPOTIFY_REGULAR: FontArc = {
        let font_data = include_bytes!("../../../../assets/fonts/SpotifyMix-Regular.ttf");
        FontArc::try_from_slice(font_data).expect("Error loading font")
    };
    pub static ref SPOTIFY_BOLD: FontArc = {
        let font_data = include_bytes!("../../../../assets/fonts/SpotifyMix-Bold.ttf");
        FontArc::try_from_slice(font_data).expect("Error loading font")
    };
    pub static ref SPOTIFY_ULTRA: FontArc = {
        let font_data = include_bytes!("../../../../assets/fonts/SpotifyMix-Ultra.ttf");
        FontArc::try_from_slice(font_data).expect("Error loading font")
    };
    pub static ref SPOTIFY_MEDIUM: FontArc = {
        let font_data = include_bytes!("../../../../assets/fonts/SpotifyMix-Medium.ttf");
        FontArc::try_from_slice(font_data).expect("Error loading font")
    };
}
