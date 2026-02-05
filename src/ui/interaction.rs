//! This module contains the logic for handling interactions
//! with the UI using picking backend observers
use super::*;
use bevy::window::CursorOptions;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(apply_palette_on_over)
        .add_observer(apply_palette_on_click)
        .add_observer(apply_palette_on_out)
        .add_observer(apply_palette_on_release)
        .add_observer(play_sound_effect_on_click)
        .add_observer(play_sound_effect_on_over);
}

fn apply_palette_on_click(
    click: On<Pointer<Click>>,
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
            t.0 = palette.pressed.text;
        }
    }
}

fn apply_palette_on_release(
    click: On<Pointer<Release>>,
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
    (*bg, *border) = (palette.hovered.bg.into(), palette.hovered.border);

    for c in &*children {
        if let Ok(mut t) = text_color_q.get_mut(*c) {
            t.0 = palette.hovered.text;
        }
    }
}

fn apply_palette_on_over(
    hover: On<Pointer<Over>>,
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
}

fn apply_palette_on_out(
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

fn play_sound_effect_on_click(
    on: On<Pointer<Click>>,
    btn_q: Query<&Button>,
    settings: Res<Settings>,
    sources: If<Res<AudioSources>>,
    cursor_opt: Query<&CursorOptions>,
    mut commands: Commands,
) {
    if btn_q.get(on.event_target()).is_ok()
        && let Ok(cursor) = cursor_opt.single()
        && cursor.visible
    {
        commands.spawn(SamplePlayer::new(sources.press.clone()).with_volume(settings.sfx()));
    }
}

fn play_sound_effect_on_over(
    on: On<Pointer<Over>>,
    btn_q: Query<&Button>,
    settings: Res<Settings>,
    sources: If<Res<AudioSources>>,
    cursor_opt: Query<&CursorOptions>,
    mut commands: Commands,
) {
    if btn_q.get(on.event_target()).is_ok()
        && let Ok(cursor) = cursor_opt.single()
        && cursor.visible
    {
        commands.spawn(SamplePlayer::new(sources.hover.clone()).with_volume(settings.sfx()));
    }
}

// TODO: adding Disabled observer
// fn on_disable(disable: On<Add, Disabled>, mut commands: Commands) {
//      // painting button gray or something
// }
