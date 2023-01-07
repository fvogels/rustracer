use crate::{
    imaging::color::Color,
    lights::light::{LightRay, LightSource},
    materials::material::MaterialProperties,
    math::Ray,
    primitives::primitive::Hit,
};

use super::scene::Scene;

pub struct RayTracer {
    pub scene: Scene,
}

pub struct TraceResult {
    pub color: Color,
}

impl RayTracer {
    pub fn new(scene: Scene) -> Self {
        RayTracer { scene }
    }

    pub fn trace(&self, ray: &Ray) -> TraceResult {
        match self.scene.root.find_first_positive_hit(ray) {
            None => TraceResult {
                color: Color::black(),
            },
            Some(hit) => match hit.material_properties {
                None => TraceResult {
                    color: Color::black(),
                },
                Some(ref material_properties) => {
                    let light_color = self.process_lights(&hit, material_properties);
                    TraceResult { color: light_color }
                }
            },
        }
    }

    fn process_lights(&self, hit: &Hit, material_properties: &MaterialProperties) -> Color {
        let mut result = Color::black();

        for light_source in self.scene.light_sources.iter() {
            result += self.process_light(hit, material_properties, light_source.as_ref());
        }

        result
    }

    fn process_light(
        &self,
        hit: &Hit,
        material_properties: &MaterialProperties,
        light_source: &dyn LightSource,
    ) -> Color {
        let mut result = Color::black();
        let mut n_lightrays = 0;

        for light_ray in light_source.lightrays_to(hit.global_position()) {
            result += self.process_light_ray(hit, material_properties, &light_ray);

            n_lightrays += 1;
        }

        result /= n_lightrays as f64;
        result
    }

    fn process_light_ray(
        &self,
        hit: &Hit,
        material_properties: &MaterialProperties,
        light_ray: &LightRay,
    ) -> Color {
        let is_shadowed = match self.scene.root.find_first_positive_hit(&light_ray.ray) {
            None => false,
            Some(ref hit) => hit.t < 0.999,
        };

        if is_shadowed {
            Color::black()
        } else {
            let cos_angle = -hit.normal().cos_angle_between(&light_ray.ray.direction);

            if cos_angle > 0.0 {
                light_ray.color * cos_angle * material_properties.diffuse
            } else {
                Color::black()
            }
        }
    }
}
