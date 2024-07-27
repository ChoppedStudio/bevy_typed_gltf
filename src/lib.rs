use std::marker::PhantomData;

use bevy_asset::{transformer::AssetTransformer, Asset};
pub use bevy_gltf::Gltf;

pub use bevy_typed_gltf_macros::TypedGltf;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("A given name or index couldn't be found in the gltf.")]
pub struct GltfTypeError;

pub trait TypedGltf: Sized {
    fn from_gltf(gltf: &Gltf) -> Result<Self, GltfTypeError>;
}

pub struct TypedGltfTransformer<T: TypedGltf>(PhantomData<T>);

impl<T: TypedGltf + Asset + Send + Sync + 'static> AssetTransformer for TypedGltfTransformer<T> {
    type AssetInput = Gltf;
    type AssetOutput = T;
    type Settings = ();
    type Error = GltfTypeError;

    async fn transform<'a>(
        &'a self,
        asset: bevy_asset::transformer::TransformedAsset<Self::AssetInput>,
        _settings: &'a Self::Settings,
    ) -> Result<bevy_asset::transformer::TransformedAsset<Self::AssetOutput>, Self::Error> {
        let typed = T::from_gltf(&asset);
        typed.map(|t| asset.replace_asset(t))
    }
}
