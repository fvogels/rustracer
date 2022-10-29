mod util;
mod cameras;
mod imaging;
mod math;
mod primitives;
mod samplers;

use cameras::perspective::{PerspectiveCamera, PerspectiveCameraParameters};
use imaging::color::Color;
use imaging::image::Image;
use math::point2d::Point2D;
use math::point3d::Point3D;
use math::position2d::Position2D;
use math::rasterizer2d::Rasterizer2D;
use math::rectangle2d::Rectangle2D;
use math::vector2d::Vector2D;
use math::vector3d::Vector3D;
use primitives::primitive::Primitive;
use primitives::sphere::Sphere;
use samplers::{sampler::Sampler2D, single::SingleSampler2D, stratified::StratifiedSampler2D};

fn main() {
    let width: u32 = 100;
    let height: u32 = 100;
    let mut image = Image::new(width, height);
    let camera_parameters = PerspectiveCameraParameters {
        aspect_ratio: 1.0,
        distance_to_screen: 1.0,
        eye: p3!(0, 0, -5),
        look_at: p3!(0, 0, 0),
        up: v3!(0, 1, 0),
    };
    let camera = PerspectiveCamera::new(&camera_parameters);
    let rectangle = Rectangle2D::new(p2!(0, 0), v2!(1, 0), v2!(0, 1));
    let rasterizer = Rasterizer2D::new(&rectangle, width, height);
    let sampler = StratifiedSampler2D::new(4, 4);
    let sphere = Sphere::new();

    for y in 0..height {
        for x in 0..width {
            let position = Position2D::new(x as i32, y as i32);
            let pixel = rasterizer.at(position);
            let mut sample_count = 0;
            let mut accumulated_color = Color::black();

            for sample in sampler.sample(&pixel) {
                sample_count += 1;

                let camera_rays = camera.enumerate_rays(sample);

                for ray in camera_rays {
                    let hit = match sphere.find_first_positive_hit(&ray) {
                        None => false,
                        Some(_) => true
                    };

                    let sample_color = if hit {
                        Color::white()
                    } else {
                        Color::black()
                    };

                    accumulated_color += &sample_color;
                }

                accumulated_color /= sample_count as f64;
                *image.get_mut(position) = accumulated_color;
            }
        }
    }

    image
        .write_to_file(std::path::Path::new(r"g:/temp/test.png"))
        .expect("Failed to write image to file");
}
