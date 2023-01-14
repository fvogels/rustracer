mod cameras;
mod data;
mod imaging;
mod lights;
mod materials;
mod math;
mod primitives;
mod regex;
mod samplers;
mod scripting;
mod tracing;
mod util;

use std::rc::Rc;

use cameras::perspective::{PerspectiveCamera, PerspectiveCameraParameters};
use imaging::color::Color;
use imaging::image::Image;
use lights::{light::LightSource, point::PointLight};
use materials::uniform::UniformMaterial;
use math::{transformation3d::Transformation3D, Position, Rasterizer, Rectangle};
use primitives::decorator::Decorator;
use primitives::sphere::Sphere;
use primitives::{primitive::Primitive, transformer::Transformer, union::Union};
use samplers::{Sampler2D, StratifiedSampler2D};
use tracing::raytracer::RayTracer;
use tracing::scene::Scene;

use crate::materials::reflective::Reflective;
use crate::primitives::plane::PlaneXY;

fn create_scene() -> Scene {
    fn create_camera() -> PerspectiveCamera {
        let camera_parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye: pt!(0, 5, 10),
            look_at: pt!(0, 0, 0),
            up: vc!(0, 1, 0),
        };

        PerspectiveCamera::new(&camera_parameters)
    }

    fn create_root() -> Rc<dyn Primitive> {
        let plane = Rc::new(PlaneXY::new());
        let sphere = Rc::new(Sphere::new());

        let background = plane;
        let left_sphere = Rc::new(Transformer::new(
            Transformation3D::translate(&vc!(-2, 0, 5)),
            sphere.clone(),
        ));
        let right_sphere = Rc::new(Transformer::new(
            Transformation3D::translate(&vc!(1, 0, 5)),
            sphere,
        ));

        let red_material = Rc::new(UniformMaterial::new(Color::red()));
        let green_material = Rc::new(UniformMaterial::new(Color::green()));
        let blue_material = Rc::new(UniformMaterial::new(Color::blue()));
        let reflective_material = Rc::new(Reflective::new(0.5));

        let background = Rc::new(Decorator::new(reflective_material, background));
        let left_sphere = Rc::new(Decorator::new(red_material, left_sphere));
        let right_sphere = Rc::new(Decorator::new(blue_material, right_sphere));

        let union = Union::new(vec![left_sphere, right_sphere, background]);

        Rc::new(union)
    }

    fn create_light_sources() -> Vec<Rc<dyn LightSource>> {
        let light = Rc::new(PointLight::new(Color::white(), pt!(0, 5, 3)));

        vec![light]
    }

    let camera = create_camera();
    let root = create_root();
    let light_sources = create_light_sources();

    Scene {
        camera,
        root,
        light_sources,
    }
}

fn main() {
    let width: u32 = 500;
    let height: u32 = 500;
    let mut image = Image::new(width, height);

    let rectangle = Rectangle::new(pt!(0, 0), vc!(1, 0), vc!(0, 1));
    let rasterizer = Rasterizer::new(rectangle, width, height);
    let sampler = StratifiedSampler2D::new();
    let scene = create_scene();
    let ray_tracer = Rc::new(RayTracer::new(scene));

    for y in 0..height {
        for x in 0..width {
            let position = Position::<2>::cartesian(x as i32, y as i32);
            let pixel = rasterizer.at(position);
            let mut sample_count = 0;
            let mut accumulated_color = Color::black();

            for sample in sampler.sample(pixel).take(5) {
                let camera_rays = ray_tracer.scene.camera.enumerate_rays(sample);

                for ray in camera_rays {
                    let trace_result = ray_tracer.trace(&ray);
                    sample_count += 1;
                    accumulated_color += &trace_result.color;
                }
            }

            accumulated_color /= sample_count as f64;
            *image.get_mut(position) = accumulated_color;
        }
    }

    image
        .write_to_file(std::path::Path::new(r"test.png"))
        .expect("Failed to write image to file");
}
