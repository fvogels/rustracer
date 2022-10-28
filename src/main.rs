mod cameras;
mod imaging;
mod math;
mod primitives;

use cameras::perspective::{PerspectiveCamera, PerspectiveCameraParameters};
use imaging::color::Color;
use imaging::image::Image;
use math::point3d::{Point3D};
use math::vector3d::{Vector3D};

fn main() {
    let width: i32 = 500;
    let height: i32 = 500;
    let mut image = Image::new(width as u32, height as u32);
    let camera_parameters = PerspectiveCameraParameters {
        aspect_ratio: 1.0,
        distance_to_screen: 1.0,
        eye: p3!(0, 0, -5),
        look_at: p3!(0, 0, 0),
        up: v3!(0, 1, 0),
    };
    let camera = PerspectiveCamera::new(&camera_parameters);



    image
        .write_to_file(std::path::Path::new(r"g:/temp/test.png"))
        .expect("Failed to write image to file");
}
