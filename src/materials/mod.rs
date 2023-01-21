mod material;
mod uniform;
mod reflective;
mod diffuse;

pub use material::{Material, MaterialResult, TraceFunction};
pub use uniform::UniformMaterial;
pub use reflective::ReflectiveMaterial;
pub use diffuse::DiffuseMaterial;
