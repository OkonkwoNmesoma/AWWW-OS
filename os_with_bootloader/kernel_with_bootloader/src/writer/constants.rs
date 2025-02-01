use noto_sans_mono_bitmap::{get_raster_width, FontWeight, RasterHeight};

pub mod font_constants {
    use super::*;

    /// Height of each character raster
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

    /// Width of each character in the monospace font
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

    /// Default backup character if a symbol is unavailable
    pub const BACKUP_CHAR: char = ' ';

    /// Font weight
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
}
