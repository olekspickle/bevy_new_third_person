//! This module contains the logic for handling interactions
//! with the UI using picking backend observers
//!
//! TODO: this is quite a lot of duplication, maybe there is a better way to structure it?..
use super::*;
use bevy::window::CursorOptions;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_hover)
        .add_observer(on_click)
        .add_observer(on_out);
}

fn on_click(
    click: On<Pointer<Click>>,
    settings: Res<Settings>,
    sources: If<Res<AudioSources>>,
    cursor_opt: Query<&CursorOptions>,
    mut commands: Commands,
    mut palette_q: Query<(
        &PaletteSet,
        &mut BorderColor,
        &mut BackgroundColor,
        &mut Children,
    )>,
    mut text_color_q: Query<&mut TextColor>,
) {
    let Ok((palette, mut border, mut bg, children)) = palette_q.get_mut(click.event_target())
    else {
        return;
    };
    (*bg, *border) = (palette.pressed.bg.into(), palette.pressed.border);

    for c in &*children {
        if let Ok(mut t) = text_color_q.get_mut(*c) {
            t.0 = palette.hovered.text;
        }
    }

    if let Ok(cursor) = cursor_opt.single() {
        if cursor.visible {
            commands.spawn(SamplePlayer::new(sources.press.clone()).with_volume(settings.sfx()));
        }
    }
}
fn on_hover(
    hover: On<Pointer<Over>>,
    settings: Res<Settings>,
    sources: If<Res<AudioSources>>,
    cursor_opt: Query<&CursorOptions>,
    mut commands: Commands,
    mut palette_q: Query<(
        &PaletteSet,
        &mut BorderColor,
        &mut BackgroundColor,
        &mut Children,
    )>,
    mut text_color_q: Query<&mut TextColor>,
) {
    let Ok((palette, mut border, mut bg, children)) = palette_q.get_mut(hover.event_target())
    else {
        return;
    };
    (*bg, *border) = (palette.hovered.bg.into(), palette.hovered.border);

    for c in &*children {
        if let Ok(mut t) = text_color_q.get_mut(*c) {
            t.0 = palette.hovered.text;
        }
    }

    if let Ok(cursor) = cursor_opt.single() {
        if cursor.visible {
            commands.spawn(SamplePlayer::new(sources.hover.clone()).with_volume(settings.sfx()));
        }
    }
}

fn on_out(
    hover: On<Pointer<Out>>,
    mut palette_q: Query<(
        &PaletteSet,
        &mut BorderColor,
        &mut BackgroundColor,
        &mut Children,
    )>,

    mut text_color_q: Query<&mut TextColor>,
) {
    let Ok((palette, mut border, mut bg, children)) = palette_q.get_mut(hover.event_target())
    else {
        return;
    };
    (*bg, *border) = (palette.none.bg.into(), palette.none.border);

    for c in &*children {
        if let Ok(mut t) = text_color_q.get_mut(*c) {
            t.0 = palette.none.text;
        }
    }
}

// TODO: adding Disabled observer
// fn on_disable(disable: On<Add, Disabled>, mut commands: Commands) {
//      // painting button gray or something
// }
