// Disable console on Windows for non-dev builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian3d::prelude::*;
use bevy::asset::load_internal_binary_asset;
use bevy::pbr::DefaultOpaqueRendererMethod;
use bevy::{
    app::App, asset::AssetMetaCheck, log, prelude::*, window::PrimaryWindow, winit::WINIT_WINDOWS,
};
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_skein::SkeinPlugin;
use winit::window::Icon;

use std::io::Cursor;

pub mod asset_loading;
pub mod audio;
pub mod camera;
pub mod game;
pub mod models;
pub mod player;
pub mod scene;
pub mod screens;
pub mod third_party;
pub mod ui;

use asset_loading::{AudioSources, Models, ResourceHandles, Textures};
use audio::*;
use models::*;
use scene::*;
pub use third_party::*;
use ui::*;

fn main() {
    let mut app = App::new();

    let filter =
        "info,bevy_new_3d_rpg=debug,calloop=off,symphonia=off,naga=off,wgpu=warn".to_string();
    // let filter = "debug,symphonia=off,naga=off,wgpu=warn,bevy_enhanced_input=debug".to_string(); // DEBUG

    app.insert_resource(DefaultOpaqueRendererMethod::deferred());
    app.add_plugins((DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game".to_string(),
                // Bind to canvas included in `index.html` for custom wasm js logic
                // canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        })
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        })
        .set(log::LogPlugin {
            level: log::Level::TRACE,
            filter,
            ..default()
        }),));

    app.add_plugins(third_party::plugin);

    // our plugins. the order is important
    // be sure you use resources/types AFTER you add plugins that insert them
    app.add_plugins((
        audio::plugin,
        asset_loading::plugin,
        camera::plugin,
        ui::plugin,
        models::plugin,
        scene::plugin,
        player::plugin,
        screens::plugin,
        game::plugin,
    ))
    .add_systems(Startup, set_window_icon);

    // override default font
    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../assets/fonts/Not-Jam-Mono-Clean-16.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}

/// Sets the icon on windows and X11
/// TODO: fix when bevy gets a normal way of setting window image
/// FIXME: use query again after !Send resources are removed
/// <https://github.com/bevyengine/bevy/issues/17667>
fn set_window_icon(
    // windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) -> Result {
    // let Some(primary) = windows.get_window(primary_entity) else {
    //     return Ok(());
    // };
    let primary_entity = primary_window.single()?;

    WINIT_WINDOWS.with_borrow_mut(|windows| {
        let Some(primary) = windows.get_window(primary_entity) else {
            return;
        };
        let icon_buf = Cursor::new(include_bytes!("../assets/textures/icon.png"));
        if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            let icon = Icon::from_rgba(rgba, width, height).unwrap();
            primary.set_window_icon(Some(icon));
        };
    });

    Ok(())
}
