use std::{ops::Neg, rc::Rc};

use crate::{
    imaging::color::Color,
    lights::light::{LightRay, LightSource},
    math::Ray,
    primitives::primitive::Hit, materials::material::TraceFunction,
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

    pub fn trace(self: &Rc<Self>, ray: &Ray) -> TraceResult {
        self.weighted_trace(ray, 1.0)
    }

    fn weighted_trace(self: &Rc<Self>, ray: &Ray, weight: f64) -> TraceResult {
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

                    let material = hit.material.as_ref();
                    let trace_function: TraceFunction = {
                        let me = self.clone();
                        let origin = hit.global_position();
                        let transformation= hit.transformation;

                        Box::new(move |direction, w| {
                            let transformed_direction = &transformation.matrix * direction;
                            let mut ray = Ray::new(origin, transformed_direction);
                            ray.nudge(0.0001);
                            me.weighted_trace(&ray, w * weight).color
                        })
                    };
                    let material_result = material.at(&-ray.direction, trace_function);
                    let color = material_result.current().clone();
                    TraceResult { color }
                }
            }
        }
    }

    // fn process_lights(&self, hit: &Hit, material_properties: &MaterialProperties) -> Color {
    //     let mut result = Color::black();

    //     for light_source in self.scene.light_sources.iter() {
    //         result += self.process_light(hit, material_properties, light_source.as_ref());
    //     }

    //     result
    // }

    // fn process_light(
    //     &self,
    //     hit: &Hit,
    //     material_properties: &MaterialProperties,
    //     light_source: &dyn LightSource,
    // ) -> Color {
    //     let mut result = Color::black();
    //     let mut n_lightrays = 0;

    //     for light_ray in light_source.lightrays_to(hit.global_position()) {
    //         result += self.process_light_ray(hit, material_properties, &light_ray);

    //         n_lightrays += 1;
    //     }

    //     result /= n_lightrays as f64;
    //     result
    // }

    // fn process_light_ray(
    //     &self,
    //     hit: &Hit,
    //     material_properties: &MaterialProperties,
    //     light_ray: &LightRay,
    // ) -> Color {
    //     let is_shadowed = match self.scene.root.find_first_positive_hit(&light_ray.ray) {
    //         None => false,
    //         Some(ref hit) => hit.t < 0.999,
    //     };

    //     if is_shadowed {
    //         Color::black()
    //     } else {
    //         let cos_angle = -hit.normal().cos_angle_between(&light_ray.ray.direction);

    //         if cos_angle > 0.0 {
    //             light_ray.color * cos_angle * material_properties.diffuse
    //         } else {
    //             Color::black()
    //         }
    //     }
    // }
}
