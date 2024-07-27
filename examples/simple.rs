use bevy_asset::Handle;
use bevy_scene::Scene;

use bevy_typed_gltf::TypedGltf;

#[derive(TypedGltf)]
struct MyGltf {
    #[gltf(scene = 0)]
    first_scene: Handle<Scene>,
    #[gltf(scene = "favourite")]
    favourite_scene: Handle<Scene>,
}

fn main() {}
