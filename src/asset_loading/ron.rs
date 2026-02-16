//! At the time of writing bevy_common_assets did not migrate to 0.16
//! and the way it restricts plugin to a single generic struct while
//! still requiring extention list doesn't really makes sense to me.
//!
//! This is a simplified ron loader from bevy_common_assets
use bevy::{
    asset::{Asset, AssetApp, AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RonLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [conversion Error](std::str::Utf8Error)
    #[error("Could not interpret as UTF-8: {0}")]
    FormatError(#[from] std::str::Utf8Error),
    /// A [Ron Error](ron::de::SpannedError)
    #[error("Could not parse RON: {0}")]
    RonError(#[from] ron::de::SpannedError),
}

/// Plugin to load your asset type `A` from ron files.
#[derive(Default)]
pub struct RonLoadPlugin<A>(PhantomData<A>);

impl<A> Plugin for RonLoadPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(RonLoader::<A>(PhantomData));
    }
}

/// Loads your asset type `A` from ron files
#[derive(TypePath)]
pub struct RonLoader<A>(PhantomData<A>);

impl<A> AssetLoader for RonLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = RonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = ron::de::from_bytes::<A>(&bytes)?;
        Ok(asset)
    }
}

// // attempt on any ron
// use bevy::app::Plugin;
// use bevy::reflect::Reflect;
// use std::ops::{Deref, DerefMut};
// use std::str::from_utf8;
//
// pub type RonValue = ron::Value;
//
// /// Representation of any Ron asset
// #[derive(Asset, Reflect)]
// pub struct Ron(
//     // Wrapped with option due to need for default implementation
//     #[reflect(ignore)] Option<RonValue>,
// );
//
// impl Deref for Ron {
//     type Target = RonValue;
//
//     fn deref(&self) -> &Self::Target {
//         self.0.as_ref().unwrap()
//     }
// }
//
// impl DerefMut for Ron {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.0.as_mut().unwrap()
//     }
// }
//
// #[derive(Default)]
// pub struct RonAssetPlugin;
// impl Plugin for RonAssetPlugin {
//     fn build(&self, app: &mut bevy::prelude::App) {
//         app.init_asset::<Ron>().register_asset_loader(RonLoader);
//     }
// }
//
// #[derive(TypePath)]
// struct RonLoader;
//
// impl AssetLoader for RonLoader {
//     type Asset = Ron;
//     type Settings = ();
//     type Error = RonLoaderError;
//
//     async fn load(
//         &self,
//         reader: &mut dyn Reader,
//         _settings: &Self::Settings,
//         _load_context: &mut bevy::asset::LoadContext<'_>,
//     ) -> Result<Self::Asset, Self::Error> {
//         let mut bytes = Vec::new();
//         reader.read_to_end(&mut bytes).await?;
//         let s = from_utf8(&bytes)?;
//         let asset = ron::de::from_str(s)?;
//         Ok(Ron(Some(asset)))
//     }
//
//     fn extensions(&self) -> &[&str] {
//         &["ron"]
//     }
// }
