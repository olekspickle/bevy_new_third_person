use crate::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen)
        .add_systems(
            Update,
            continue_to_menu_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
        );
}

fn setup_perf_ui(mut commands: Commands) {
    commands
        .container(
            FlexDirection::Column,
            AlignItems::Start,
            JustifyContent::Start,
        )
        .with_children(|children| {
            children.label(format!("Score: {}", score.0), LayoutOpts::label());
        });
}
