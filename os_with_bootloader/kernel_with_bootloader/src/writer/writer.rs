use super::constants::font_constants;
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use noto_sans_mono_bitmap::get_raster;

pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl FrameBufferWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut writer = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        writer.clear();
        writer
    }

    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.fill(0);
    }

    pub fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + 2;
            self.x_pos = 0;
            return;
        }

        let rasterized = get_raster(c, font_constants::FONT_WEIGHT, font_constants::CHAR_RASTER_HEIGHT)
            .unwrap_or_else(|| get_raster(font_constants::BACKUP_CHAR, font_constants::FONT_WEIGHT, font_constants::CHAR_RASTER_HEIGHT).unwrap());

        for (y, row) in rasterized.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }

        self.x_pos += font_constants::CHAR_RASTER_WIDTH + 1;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            _ => [intensity, intensity, intensity, 0],
        };

        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
    }
}

use core::fmt::{self, Write}; // Ensure fmt::Write is imported

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
