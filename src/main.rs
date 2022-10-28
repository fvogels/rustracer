mod imaging;
mod math;
mod primitives;
mod cameras;

use imaging::color::Color;
use imaging::image::Image;

fn main() {
    let width: i32 = 500;
    let height: i32 = 500;
    let mut image = Image::new(width as u32, height as u32);
    let cx = (width / 2) as i32;
    let cy = (height / 2) as i32;

    for y in 0..height {
        for x in 0..width {
            let dx = cx - x;
            let dy = cy - y;
            let d = dx * dx + dy * dy;

            if d < 1000 {
                *(image.get_mut(x as usize, y as usize)) = Color::white();
            }
        }
    }

    image
        .write_to_file(std::path::Path::new(r"g:/temp/test.png"))
        .expect("Failed to write image to file");
}
