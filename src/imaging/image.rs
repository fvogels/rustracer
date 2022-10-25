use crate::imaging::color::Color;

pub struct Image {
    pixels: Vec<Color>,
    width: u32,
}

impl Image {
    fn new(width: u32, height: u32) -> Image {
        let pixel_count = width * height;
        let pixels = Vec::new();
        pixels.resize(pixel_count as usize, Color::black());

        Image { pixels, width }
    }
}