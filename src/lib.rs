pub use bevy_gltf::Gltf;

pub use bevy_typed_gltf_macros::TypedGltf;

pub struct GltfTypeError;

pub trait TypedGltf: Sized {
    fn from_gltf(gltf: Gltf) -> Result<Self, GltfTypeError>;
}
