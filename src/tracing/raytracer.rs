use std::{ops::Neg, rc::Rc};

use crate::{
    imaging::color::Color,
    lights::light::{LightRay, LightSource},
    math::{Ray, Matrix},
    primitives::Hit, samplers::HemisphereSampler, util::Refine, materials::MaterialProperties,
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
        self.weighted_trace(ray, 1.0)
    }

    fn weighted_trace(&self, ray: &Ray, weight: f64) -> TraceResult {
        if weight < 0.01 {
            TraceResult { color: Color::black() }
        } else {
            match self.scene.root.find_first_positive_hit(ray) {
                None => {
                    TraceResult {
                        color: Color::black(),
                    }
                }
                Some(hit) => {
                    debug_assert!(hit.t > 0.0, "find_first_positive_hit returned hit with negative t-value: {}", hit.t);

                    TraceResult {
                        color: self.determine_color(hit, weight)
                    }
                }
            }
        }
    }

    fn determine_color(&self, hit: Hit, weight: f64) -> Color {
        match &hit.material_properties {
            None => Color::black(),
            Some(material_properties) => {
                self.direct_illumination(&hit, material_properties) + self.reflection(&hit, material_properties, weight) + self.indirect_illumination(&hit, material_properties, weight)
            }
        }
    }

    fn reflection(&self, hit: &Hit, material_properties: &MaterialProperties, weight: f64) -> Color {
        let reflection = &material_properties.reflection;

        if reflection.is_not_black() {
            let mut reflected_ray = {
                let reflected_direction = hit.ray.direction.reflect(&hit.normal());
                Ray::new(hit.global_position(), reflected_direction)
            }.nudged(0.00001);
            self.weighted_trace(&reflected_ray, weight * reflection.intensity()).color * material_properties.reflection
        } else {
            Color::black()
        }
    }

    fn direct_illumination(&self, hit: &Hit, material_properties: &MaterialProperties) -> Color {
        self.process_lights(hit, material_properties)
    }

    fn indirect_illumination(&self, hit: &Hit, material_properties: &MaterialProperties, weight: f64) -> Color {
        match &material_properties.brdf {
            None => {
                Color::black()
            }
            Some(brdf) => {
                let ray = &hit.ray;
                let mut sampler = HemisphereSampler::new();
                let mut accumulated_color = Color::black();
                let mut total_weight = 0.0;
                let sample_count = 100;

                for _ in 0..sample_count {
                    let origin = hit.global_position();
                    let direction = sampler.current();
                    let transformed_direction = &hit.transformation.matrix * &direction;
                    let mut ray = Ray::new(origin, transformed_direction).nudged(0.00001);

                    let brdf_factor = {
                        let outgoing = &hit.transformation.inverse_matrix * &ray.direction.normalized();
                        let incoming = &direction;
                        brdf.compute(&outgoing, incoming)
                    };
                    total_weight += brdf_factor;
                    accumulated_color += self.weighted_trace(&ray, weight * 0.09).color * direction.dot(&hit.normal()) * brdf_factor;
                    sampler.refine();
                }

                accumulated_color / total_weight
            }
        }
    }

    fn process_lights(&self, hit: &Hit, material_properties: &MaterialProperties) -> Color {
        let mut result = Color::black();

        for light_source in self.scene.light_sources.iter() {
            result += self.process_light(hit, light_source.as_ref(), material_properties);
        }

        result
    }

    fn process_light(
        &self,
        hit: &Hit,
        light_source: &dyn LightSource,
        material_properties: &MaterialProperties,
    ) -> Color {
        let mut result = Color::black();
        let mut n_lightrays = 0;

        for light_ray in light_source.lightrays_to(hit.global_position()) {
            result += self.process_light_ray(hit, &light_ray, material_properties);

            n_lightrays += 1;
        }

        result /= n_lightrays as f64;
        result
    }

    fn process_light_ray(
        &self,
        hit: &Hit,
        light_ray: &LightRay,
        material_properties: &MaterialProperties,
    ) -> Color {
        let is_shadowed = match self.scene.root.find_first_positive_hit(&light_ray.ray) {
            None => false,
            Some(ref hit) => hit.t < 0.999,
        };

        if is_shadowed {
            Color::black()
        } else {
            self.compute_diffuse_lighting(hit, material_properties, light_ray) + self.compute_specular_lighting(hit, material_properties, light_ray)
        }
    }

    fn compute_diffuse_lighting(&self, hit: &Hit, material_properties: &MaterialProperties, light_ray: &LightRay) -> Color {
        let cos_angle = -hit.normal().cos_angle_between(&light_ray.ray.direction);

        if cos_angle > 0.0 {
            light_ray.color * cos_angle * material_properties.diffuse
        } else {
            Color::black()
        }
    }

    fn compute_specular_lighting(&self, hit: &Hit, material_properties: &MaterialProperties, light_ray: &LightRay) -> Color {
        let reflected_light_direction = light_ray.ray.direction.reflect(&hit.normal()).normalized();
        let eye_direction = {
            let eye = &hit.ray.origin;
            (eye - &hit.global_position()).normalized()
        };

        light_ray.color * material_properties.specular_color * reflected_light_direction.dot(&eye_direction).powf(material_properties.specular_exponent)
    }
}
