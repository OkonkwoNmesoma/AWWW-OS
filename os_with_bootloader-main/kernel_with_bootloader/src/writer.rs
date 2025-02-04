mod constants;

use core::{
    fmt::{self, Arguments, Write},
    ptr,
};

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use lazy_static::lazy_static;
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};
use spin::Mutex;

// use self::constants::Colour;

/// Additional vertical space between lines
const LINE_SPACING: usize = 2;

/// Additional horizontal space between characters.
const LETTER_SPACING: usize = 0;

/// Padding from the border. Prevent that font is too close to border.
const BORDER_PADDING: usize = 1;

/// Returns the raster of the given char or the raster of [`font_constants::BACKUP_CHAR`].
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

lazy_static! {
    pub static ref FRAME_BUFFER_WRITER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);
}

/// Allows logging text to a pixel-based framebuffer.
pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    // color: Colour,
}

impl FrameBufferWriter {
    /// Creates a new logger that uses the given framebuffer.

    /// Creates a new logger that uses the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
            // color: Colour::LightBlue,
        };
        logger.clear();
        logger
    }

    fn newline(&mut self) {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.x_pos += x;
        self.y_pos += y;
    }

    pub fn tab(&mut self, x: usize) {
        self.x_pos += x;
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    pub fn back_space(&mut self) {
        if self.x_pos > BORDER_PADDING {
            self.x_pos -= font_constants::CHAR_RASTER_WIDTH + LETTER_SPACING;
        } else {
            if self.y_pos
                >= font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING + BORDER_PADDING
            {
                self.y_pos -= font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
                self.x_pos = self.width() - BORDER_PADDING;
            } else {
                return;
            }
        }

        for u in 0..font_constants::CHAR_RASTER_HEIGHT.val() {
            for w in 0..font_constants::CHAR_RASTER_WIDTH {
                self.write_pixel(self.x_pos + w, self.y_pos + u, 0);
            }
        }
    }

    pub fn cursor_up(&mut self, move_amount: usize) {
        if self.y_pos >= move_amount {
            self.y_pos -= move_amount;
        } else {
            self.y_pos = 0;
        }
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_up(font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING)
    }

    pub fn cursor_down(&mut self, move_amount: usize) {
        let max_y = self.height() - BORDER_PADDING;
        if self.y_pos + move_amount <= max_y {
            self.y_pos += move_amount;
        } else {
            self.y_pos = max_y;
        }
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_down(font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING)
    }

    pub fn cursor_left(&mut self, move_amount: usize) {
        if self.x_pos >= move_amount {
            self.x_pos -= move_amount;
        } else {
            self.x_pos = 0;
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_left(font_constants::CHAR_RASTER_WIDTH + LETTER_SPACING)
    }

    pub fn cursor_right(&mut self, move_amount: usize) {
        let max_x = self.width() - BORDER_PADDING;
        if self.x_pos + move_amount <= max_x {
            self.x_pos += move_amount;
        } else {
            self.x_pos = BORDER_PADDING;
            self.move_cursor_down()
        }
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_right(font_constants::CHAR_RASTER_WIDTH + LETTER_SPACING)
    }

    /// Erases all text on the screen. Resets `self.x_pos` and `self.y_pos`.
    pub fn clear(&mut self) {
        self.set_position(BORDER_PADDING, BORDER_PADDING);
        self.framebuffer.fill(0);
    }

    /// Writes a single char to the framebuffer. Takes care of special control characters, such as
    /// newlines and carriage returns.
    fn write_char(&mut self, c: char) {
        match c {
            '\t' => self.tab(30usize),
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= self.width() {
                    self.newline();
                }
                let new_ypos =
                    self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    /// Prints a rendered char into the framebuffer.
    /// Updates `self.x_pos`.
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.move_cursor_right();
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                // set a supported (but invalid) pixel format before panicking to avoid a double
                // panic; it might not be readable though
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

//sets the FrameBufferWriter instance
pub fn set_frame_buffer_writer(writer: FrameBufferWriter) {
    // unsafe {
    *FRAME_BUFFER_WRITER.lock() = Some(writer);
    // }
}

// Move the position of the framebufferwriter
pub fn move_writer_position(horizontal_position: usize, vertical_position: usize) {
    if let Some(frame_buffer_writer) = FRAME_BUFFER_WRITER.lock().as_mut() {
        frame_buffer_writer.set_position(horizontal_position, vertical_position)
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n",format_args!($($arg)*)));
}

#[macro_export]
macro_rules! input_char {
    () => {{
        let result;
        loop {
            unsafe {
                if let Some(ch) = crate::interrupts::LAST_CHAR {
                    result = ch;
                    crate::interrupts::LAST_CHAR = None;
                    break;
                }
            }
        }

        result
    }};
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        if let Some(frame_buffer_writer) = FRAME_BUFFER_WRITER.lock().as_mut() {
            frame_buffer_writer.write_fmt(args).unwrap();
        }
    })
}

// static mut FRAME_BUFFER_WRITER: Option<FrameBufferWriter> = None;

// /// Gets the mutable reference to the FrameBufferWriter instance
// pub fn get_frame_buffer_writer() -> &'static mut FrameBufferWriter {
//     unsafe {
//         FRAME_BUFFER_WRITER
//             .as_mut()
//             .expect("FrameBufferWriter not initialized")
//     }
// }

// /// Sets the FrameBufferWriter instance
// pub fn set_frame_buffer_writer(writer: FrameBufferWriter) {
//     unsafe {
//         FRAME_BUFFER_WRITER = Some(writer);
//     }
// }

// /// Macro to print a string to the global FrameBufferWriter instance.
// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => {
//         // Use the global FrameBufferWriter instance to print the string
//         let writer = $crate::writer::get_frame_buffer_writer();
//         // self::writer::get_frame_buffer_writer();
//         // writer.write_str($s).unwrap();
//         write!(writer, $($arg)*).unwrap();
//     };
// }

// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => {
//         // Use the global FrameBufferWriter instance to print the string
//         let writer = $crate::writer::get_frame_buffer_writer();
//         // writer.write_str($s).unwrap();
//         write!(writer, $s).unwrap();
//     };
// }
