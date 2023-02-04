mod material;
mod uniform;
// mod reflective;
// mod diffuse;
// mod transform;
mod brdf;

pub use material::{Material, MaterialProperties};
pub use uniform::UniformMaterial;
// pub use reflective::ReflectiveMaterial;
// pub use diffuse::DiffuseMaterial;
// pub use transform::MaterialTransformer;
pub use brdf::BRDF;
