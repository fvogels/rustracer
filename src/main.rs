mod animation;
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

#[cfg(test)]
mod tests;

use std::rc::Rc;

use animation::{Animation, LinearAnimation, Duration, TimeStamp, TimeLine};
use cameras::perspective::{PerspectiveCamera, PerspectiveCameraParameters};
use imaging::{PNGWriter, PNGWriterOptions};
use imaging::color::Color;
use imaging::image::Image;
use lights::{light::LightSource, point::PointLight};
use materials::{UniformMaterial};
use math::{transformation3d::Transformation3D, Position, Rasterizer, Rectangle};
use primitives::{Primitive, Transformer, Union, PlaneXY, Sphere, Decorator};
use samplers::{Sampler2D, StratifiedSampler2D};
use tracing::raytracer::RayTracer;
use tracing::scene::Scene;

use crate::materials::{MaterialProperties, Material};


struct TestScene { }

impl TestScene {
    fn new() -> Self {
        TestScene { }
    }

    fn create_camera(t: TimeStamp) -> PerspectiveCamera {
        let eye_x = LinearAnimation {
            start: -2.0,
            end: 2.0,
            duration: Duration::from_seconds(1.0)
        }.at(t);

        let camera_parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye: pt!(eye_x, 5, 10),
            look_at: pt!(0, 0, 0),
            up: vc!(0, 1, 0),
        };

        PerspectiveCamera::new(&camera_parameters)
    }

    fn create_root() -> Rc<dyn Primitive> {
        fn create_material(color: Color) -> Rc<dyn Material> {
            let material_properties = MaterialProperties {
                diffuse: color,
                brdf: None,
                reflection: Color::white() * 0.8,
            };

            Rc::new(UniformMaterial::new(material_properties))
        }

        let plane = Rc::new(PlaneXY::new());
        let sphere = Rc::new(Sphere::new());

        let background = plane;
        let left_sphere = Rc::new(Transformer::new(
            Transformation3D::translate(&vc!(-2, 0, 3)),
            sphere.clone(),
        ));
        let right_sphere = Rc::new(Transformer::new(
            Transformation3D::translate(&vc!(1, 0, 5)),
            sphere,
        ));

        let red_material = create_material(Color::red());
        let green_material = create_material(Color::green());
        let blue_material = create_material(Color::blue());
        let white_material = create_material(Color::white());

        let background = Rc::new(Decorator::new(white_material.clone(), background));
        let left_sphere = Rc::new(Decorator::new(red_material, left_sphere));
        let right_sphere = Rc::new(Decorator::new(blue_material, right_sphere));

        let union = Union::new(vec![
            left_sphere,
            right_sphere,
            background,
        ]);

        Rc::new(union)
    }

    fn create_light_sources() -> Vec<Rc<dyn LightSource>> {
        let light = Rc::new(PointLight::new(Color::white(), pt!(0, 5, 1)));

        vec![light]
    }
}

impl Animation<Scene> for TestScene {
    fn duration(&self) -> Duration {
        Duration::from_seconds(1.0)
    }

    fn at(&self, t: TimeStamp) -> Scene {
        let camera = Self::create_camera(t);
        let root = Self::create_root();
        let light_sources = Self::create_light_sources();

        Scene {
            camera,
            root,
            light_sources,
        }
    }
}

struct Renderer {
    width: u32,
    height: u32,
    scene: Box<dyn Animation<Scene>>,
}

impl Renderer {
    fn new(width: u32, height: u32, scene: Box<dyn Animation<Scene>>) -> Self {
        Renderer {
            width,
            height,
            scene,
        }
    }

    fn create_rasterizer(&self) -> Rasterizer<2> {
        let rectangle = Rectangle::new(pt!(0, 0), vc!(1, 0), vc!(0, 1));

        Rasterizer::new(rectangle, self.width, self.height)
    }

    fn create_sampler(&self) -> impl Sampler2D {
        StratifiedSampler2D::new()
    }

    fn render_frame(&self, t: TimeStamp) -> Image {
        let width = self.width;
        let height = self.height;
        let mut image = Image::new(width, height);

        let rasterizer = self.create_rasterizer();
        let sampler = self.create_sampler();
        let scene = self.scene.at(t);
        let ray_tracer = Rc::new(RayTracer::new(scene));
        let sample_count = 1;

        for y in 0..height {
            for x in 0..width {
                let position = Position::<2>::cartesian(x as i32, y as i32);
                let pixel = rasterizer.at(position);
                let mut accumulated_color = Color::black();
                let mut samples = sampler.sample(pixel);

                for _ in 0..sample_count {
                    let sample = samples.current();
                    let camera_rays = ray_tracer.scene.camera.enumerate_rays(sample);

                    for ray in camera_rays {
                        let trace_result = ray_tracer.trace(&ray);
                        accumulated_color += &trace_result.color;
                    }

                    samples.refine();
                }

                accumulated_color /= sample_count as f64;
                *image.get_mut(position) = accumulated_color;
            }
        }

        image
    }
}

fn main() {
    let width = 500;
    let height = 500;
    let frames_per_second = 1;
    let path = "movie.png";
    let scene = Box::new(TestScene::new());
    let timeline = TimeLine::new(scene.duration(), frames_per_second);
    let renderer = Renderer::new(width, height, scene);
    let mut png_writer = {
        let png_options = PNGWriterOptions {
            width,
            height,
            frame_count: timeline.frame_count(),
        };

        PNGWriter::to_file(path, png_options)
    };

    for (idx, t) in timeline.iter() {
        println!("Rendering frame {idx}");
        let image = renderer.render_frame(t);
        png_writer.write_frame(image);
    }
}
