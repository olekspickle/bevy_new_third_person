use bevy::prelude::*;

use crate::Screen;

/// This plugin handles loading and saving scenes
/// Scene logic is only active during the State `GameState::Playing`
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), Scene::setup);
}

#[derive(Component)]
pub struct Scene;

impl Scene {
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // circular floor
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(400.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(55, 200, 55))),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ));

        // cube
        let mesh = Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0)));
        let color: MeshMaterial3d<StandardMaterial> =
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255)));
        commands.spawn((mesh, color, Transform::from_xyz(0.0, 0.5, 0.0)));

        // light
        commands.insert_resource(AmbientLight {
            brightness: 1000.,
            ..default()
        });
        commands.spawn((
            PointLight {
                shadows_enabled: true,
                color: Color::srgb(0.3, 0.5, 0.5),
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0),
        ));
    }

    // This system logs all Mesh3d components in our world. Try making a change to a ComponentA in
    // load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
    // the changes appear in the console whenever you make a change.
    //fn log_system(
    //    query: Query<(Entity, &Mesh3d), Changed<Mesh3d>>,
    //    res: Option<Res<MeshMaterial3d<StandardMaterial>>>,
    //) {
    //    for (entity, mesh) in &query {
    //        info!("  Entity({})", entity.index());
    //        info!("    Mesh: {{ x: {} y: {} }}\n", mesh.x, mesh.y);
    //    }
    //    if let Some(res) = res {
    //        if res.is_added() {
    //            info!("  New ResourceA: {{ score: {} }}\n", res.score);
    //        }
    //    }
    //}
}
