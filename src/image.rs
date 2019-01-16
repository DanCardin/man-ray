use image::{ImageBuffer, Rgb, RgbImage, Rgba};
use std::io;
use std::path::Path;

use crate::color::Color;

impl From<&Color> for Rgb<u8> {
    fn from(color: &Color) -> Self {
        Self {
            data: [
                (color.red * 255.99) as u8,
                (color.green * 255.99) as u8,
                (color.blue * 255.99) as u8,
            ],
        }
    }
}

impl From<&Color> for Rgba<u8> {
    fn from(color: &Color) -> Self {
        Self {
            data: [
                (color.red * 255.99) as u8,
                (color.green * 255.99) as u8,
                (color.blue * 255.99) as u8,
                255,
            ],
        }
    }
}

pub fn write_image(pixels: &[Color], aspect: f64, scale: usize, filename: &str) -> io::Result<()> {
    let width = scale;
    let height = (scale as f64 / aspect) as usize;

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    for (y, row) in pixels.chunks(width).enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            img.put_pixel(x as u32, y as u32, pixel.into());
        }
    }
    img.save(Path::new(filename))
}
