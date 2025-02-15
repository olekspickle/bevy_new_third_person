use bevy::prelude::*;

use crate::Screen;

/// This plugin handles loading and saving scenes
/// Scene logic is only active during the State `GameState::Playing`
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), setup);
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    let mesh = Mesh3d(meshes.add(Cuboid::new(2000., 2., 2000.)));
    let mat = MeshMaterial3d(materials.add(Color::srgb(0.3, 0.9, 0.3)));
    commands.spawn((mesh, mat, Transform::default()));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });
    commands.spawn((
        PointLight {
            color: Color::srgb(0.3, 0.5, 0.5),
            radius: 30.0,
            range: 100.,
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
