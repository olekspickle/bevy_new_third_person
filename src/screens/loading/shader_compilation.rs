//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use super::*;
use bevy::render::render_resource::{CachedPipelineState, PipelineCache};
use bevy::render::{MainWorld, RenderApp};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<LoadedPipelineCount>();
    app.sub_app_mut(RenderApp)
        .add_systems(ExtractSchedule, update_loaded_pipeline_count);
    app.add_systems(
        OnEnter(LoadingScreen::Shaders),
        spawn_or_skip_shader_compilation_loading_screen,
    );

    app.add_systems(
        Update,
        (
            update_loading_shaders_label,
            enter_spawn_level_screen.run_if(all_pipelines_loaded),
        )
            .chain()
            .run_if(in_state(LoadingScreen::Shaders)),
    );
}

fn spawn_or_skip_shader_compilation_loading_screen(
    mut commands: Commands,
    loaded_pipeline_count: Res<LoadedPipelineCount>,
    mut next_screen: ResMut<NextState<LoadingScreen>>,
) {
    if loaded_pipeline_count.is_done() {
        next_screen.set(LoadingScreen::Level);
        return;
    }
    commands.spawn((
        widget::ui_root("Loading Screen"),
        BackgroundColor(colors::TRANSLUCENT),
        DespawnOnExit(LoadingScreen::Shaders),
        children![(widget::label("Compiling shaders..."), LoadingShadersLabel)],
    ));
}

fn enter_spawn_level_screen(mut next_screen: ResMut<NextState<LoadingScreen>>) {
    next_screen.set(LoadingScreen::Level);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingShadersLabel;

fn update_loading_shaders_label(
    mut query: Query<&mut Text, With<LoadingShadersLabel>>,
    loaded_pipeline_count: Res<LoadedPipelineCount>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!(
            "Compiling shaders: {} / {}",
            loaded_pipeline_count.0,
            LoadedPipelineCount::TOTAL_PIPELINES
        );
    }
}

/// A `Resource` in the main world that stores the number of pipelines that are ready.
#[derive(Resource, Default, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
struct LoadedPipelineCount(pub(crate) usize);

impl LoadedPipelineCount {
    fn is_done(&self) -> bool {
        self.0 >= Self::TOTAL_PIPELINES
    }

    /// These numbers have to be tuned by hand, unfortunately.
    /// Find them out with `bevy run` and `bevy run web`.
    const TOTAL_PIPELINES: usize = {
        let count = {
            #[cfg(feature = "native")]
            {
                7
            }
            #[cfg(feature = "web")]
            {
                5
            }
        };
        #[cfg(debug_assertions)]
        {
            count
        }
        #[cfg(not(debug_assertions))]
        {
            count - 1
        }
    };
}

fn update_loaded_pipeline_count(mut main_world: ResMut<MainWorld>, cache: Res<PipelineCache>) {
    if let Some(mut pipelines_ready) = main_world.get_resource_mut::<LoadedPipelineCount>() {
        let count = cache
            .pipelines()
            .filter(|pipeline| matches!(pipeline.state, CachedPipelineState::Ok(_)))
            .count();

        if pipelines_ready.0 == count {
            return;
        }

        pipelines_ready.0 = count;
    }
}

fn all_pipelines_loaded(loaded_pipeline_count: Res<LoadedPipelineCount>) -> bool {
    loaded_pipeline_count.is_done()
}
