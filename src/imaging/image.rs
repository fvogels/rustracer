use crate::imaging::color::Color;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Image {
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub enum WriteError {
    IOError(std::io::Error),
    PNGError(png::EncodingError),
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let pixel_count = width * height;
        let mut pixels = Vec::new();
        pixels.resize(pixel_count as usize, Color::black());

        Image {
            pixels,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Color {
        let index = self.index_of(x, y);
        &self.pixels[index]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Color {
        let index = self.index_of(x, y);
        &mut self.pixels[index]
    }

    fn index_of(&self, x: usize, y: usize) -> usize {
        self.width as usize * y + x
    }

    pub fn write_to_file(&self, path: &Path) -> std::result::Result<(), WriteError> {
        fn create_encoder<'a>(
            writer: BufWriter<File>,
            width: u32,
            height: u32,
        ) -> png::Encoder<'a, BufWriter<File>> {
            let mut encoder = png::Encoder::new(writer, width, height);

            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);

            encoder
        }

        let width = self.width;
        let height = self.height;
        let file = File::create(path).map_err(WriteError::IOError)?;
        let writer = BufWriter::new(file);
        let encoder = create_encoder(writer, width, height);
        let mut writer2 = encoder.write_header().map_err(WriteError::PNGError)?;
        let data = self.convert_to_raw_rgb();
        writer2
            .write_image_data(&data)
            .map_err(WriteError::PNGError)?;

        Ok(())
    }

    fn convert_to_raw_rgb(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let bytes_per_pixel = 3;
        let total_byte_count = (bytes_per_pixel * self.width * self.height) as usize;
        result.reserve(total_byte_count);

        for color in self.pixels.iter() {
            for c in color.to_byte_array() {
                result.push(c);
            }
        }

        result
    }
}
