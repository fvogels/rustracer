mod cameras;
mod imaging;
mod math;
mod primitives;
mod samplers;
mod util;
mod materials;
mod lights;

use std::rc::Rc;

use cameras::perspective::{PerspectiveCamera, PerspectiveCameraParameters};
use imaging::color::Color;
use imaging::image::Image;
use materials::material::MaterialProperties;
use materials::uniform::UniformMaterial;
use math::{position2d::Position2D, rasterizer2d::Rasterizer2D, rectangle2d::Rectangle2D, transformation3d::Transformation3D};
use primitives::decorator::Decorator;
use primitives::{primitive::Primitive, transformer::Transformer, union::Union};
use primitives::sphere::Sphere;
use samplers::{sampler::Sampler2D, stratified::StratifiedSampler2D};

fn create_scene() -> Rc<impl Primitive> {
    let sphere = Rc::new(Sphere::new());

    let left_sphere = Rc::new(Transformer::new(Transformation3D::translate(&v3!(-1, 0, 0)), sphere.clone()));
    let right_sphere = Rc::new(Transformer::new(Transformation3D::translate(&v3!(1, 0, 0)), sphere.clone()));

    let red_material = Rc::new(UniformMaterial::new(MaterialProperties { color: Color::red() }));
    let blue_material = Rc::new(UniformMaterial::new(MaterialProperties { color: Color::blue() }));

    let left_sphere = Rc::new(Decorator::new(red_material, left_sphere));
    let right_sphere = Rc::new(Decorator::new(blue_material, right_sphere));

    let union = Union::new(vec![left_sphere, right_sphere]);

    Rc::new(union)
}

fn main() {
    let width: u32 = 500;
    let height: u32 = 500;
    let mut image = Image::new(width, height);
    let camera_parameters = PerspectiveCameraParameters {
        aspect_ratio: 1.0,
        distance_to_screen: 1.0,
        eye: p3!(0, 0, -3),
        look_at: p3!(0, 0, 0),
        up: v3!(0, 1, 0),
    };
    let camera = PerspectiveCamera::new(&camera_parameters);
    let rectangle = Rectangle2D::new(p2!(0, 0), v2!(1, 0), v2!(0, 1));
    let rasterizer = Rasterizer2D::new(&rectangle, width, height);
    let sampler = StratifiedSampler2D::new(2, 2);
    let scene = create_scene();

    for y in 0..height {
        for x in 0..width {
            let position = Position2D::new(x as i32, y as i32);
            let pixel = rasterizer.at(position);
            let mut sample_count = 0;
            let mut accumulated_color = Color::black();

            for sample in sampler.sample(&pixel) {
                let camera_rays = camera.enumerate_rays(sample);

                for ray in camera_rays {
                    let sample_color = match scene.find_first_positive_hit(&ray) {
                        None => Color::black(),
                        Some(hit) => {
                            hit.material.map(|m| m.at(hit.position.local).color).unwrap_or_else(|| Color::black())
                        },
                    };

                    sample_count += 1;
                    accumulated_color += &sample_color;
                }
            }

            accumulated_color /= sample_count as f64;
            *image.get_mut(position) = accumulated_color;
        }
    }

    image
        .write_to_file(std::path::Path::new(r"g:/temp/test.png"))
        .expect("Failed to write image to file");
}
