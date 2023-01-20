use std::{io::{BufWriter, Write}, fs::File};

use png::{Encoder, Writer};

use super::image::Image;


pub struct PNGWriter<T: Write> {
    writer: Writer<T>,
}

pub struct PNGWriterOptions {
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

impl<T: Write> PNGWriter<T> {
    pub fn new(writer: T, options: PNGWriterOptions) -> Self {
        let encoder = {
        let mut encoder = png::Encoder::new(writer, options.width, options.height);
            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_animated(options.frame_count, 0).expect("Failed to set PNG to animated");
            encoder
        };

        let writer = encoder.write_header().expect("Failed to write header");

        PNGWriter { writer }
    }

    pub fn write_frame(&mut self, image: Image) {
        let data = image.convert_to_raw_rgb();
        self.writer.write_image_data(&data).expect("Failed to write image data");
    }
}

impl<'a> PNGWriter<BufWriter<File>> {
    pub fn to_file(path: &str, options: PNGWriterOptions) -> Self {
        let file = File::create(path).expect("Failed to create file");
        let writer = BufWriter::new(file);

        Self::new(writer, options)
    }
}
