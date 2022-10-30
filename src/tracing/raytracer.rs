use crate::{imaging::color::Color, math::ray::Ray, lights::light::{LightSource, LightRay}, primitives::primitive::Hit};

use super::scene::Scene;

pub struct RayTracer {
    pub scene: Scene,
}

pub struct TraceResult {
    pub color: Color,
}

impl RayTracer {
    pub fn new(scene: Scene) -> RayTracer {
        RayTracer { scene }
    }

    pub fn trace(&self, ray: &Ray) -> TraceResult {
        match self.scene.root.find_first_positive_hit(ray) {
            None => TraceResult { color: Color::black() },
            Some(hit) => {
                match hit.material {
                    None => TraceResult { color: Color::black() },
                    Some(ref material) => {
                        let total_light = self.process_lights(&hit);
                        let object_color = material.at(hit.position.local).color;
                        let color = total_light * object_color;
                        TraceResult { color }
                    }
                }
            }
        }
    }

    fn process_lights(&self, hit: &Hit) -> Color {
        let mut result = Color::black();

        for light_source in self.scene.light_sources.iter() {
            result += self.process_light(hit, light_source.as_ref());
        }

        result
    }

    fn process_light(&self, hit: &Hit, light_source: &dyn LightSource) -> Color {
        let mut result = Color::black();
        let mut n_lightrays = 0;

        for light_ray in light_source.lightrays_to(hit.position.global) {
            result += self.process_light_ray(hit, &light_ray);

            n_lightrays += 1;
        }

        result /= n_lightrays as f64;
        result
    }

    fn process_light_ray(&self, hit: &Hit, light_ray: &LightRay) -> Color {
        let cos_angle = -hit.normal.cos_angle_between(&light_ray.ray.direction);

        if cos_angle > 0.0 {
            light_ray.color * cos_angle
        } else {
            Color::black()
        }
    }
}