mod material;
mod uniform;
mod reflective;
mod diffuse;
mod transform;

pub use material::{Material, MaterialResult, TraceFunction};
pub use uniform::UniformMaterial;
pub use reflective::ReflectiveMaterial;
pub use diffuse::DiffuseMaterial;
pub use transform::MaterialTransformer;
