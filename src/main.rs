// Disable console on Windows for non-dev builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian3d::prelude::*;
use bevy::log::tracing_subscriber::{field::MakeExt, fmt};
use bevy::{
    app::App, asset::AssetMetaCheck, asset::load_internal_binary_asset, ecs::error::error, log,
    prelude::*, window::PrimaryWindow, winit::WINIT_WINDOWS,
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
use game::*;
use models::*;
use scene::*;
use screens::*;
use third_party::*;
use ui::*;

fn main() {
    let mut app = App::new();
    // Don't panic on Bevy system errors, just log them.
    app.set_error_handler(error);

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy 3D Game".to_string(),
                    fit_canvas_to_parent: true,
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
                filter: format!(
                    // bevy_math spams `Dir3::new_unchecked` on extreme vectors
                    "info,bevy_new_3d_rpg=debug,bevy_math=error,{},",
                    bevy::log::DEFAULT_FILTER
                ),
                fmt_layer: |_| {
                    Some(Box::new(
                        fmt::Layer::default()
                            .without_time()
                            .map_fmt_fields(MakeExt::debug_alt)
                            .with_writer(std::io::stderr),
                    ))
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

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
    info!("setting window icon");
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
