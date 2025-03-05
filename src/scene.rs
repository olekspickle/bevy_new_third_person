use crate::prelude::*;
use bevy::prelude::*;
use rand::Rng;

/// This plugin handles loading and saving scenes
/// Scene logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), setup);
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    let mesh = Mesh3d(meshes.add(Cuboid::new(2000., 0., 2000.)));
    let mat = MeshMaterial3d(materials.add(Color::srgb(0.3, 0.9, 0.3)));
    commands.spawn((mesh, mat, Transform::default()));

    // Some environment
    let mesh = Mesh3d(meshes.add(Cuboid::new(200., 500., 200.)));
    let mat = MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.8)));
    commands.spawn((mesh, mat, Transform::from_xyz(-200., 250., -200.)));

    let mesh = Mesh3d(meshes.add(Cuboid::new(60., 60., 60.)));
    let mat = MeshMaterial3d(materials.add(Color::srgb(0.6, 0.3, 0.6)));
    commands.spawn((mesh, mat, Transform::from_xyz(200., 30.0, 200.)));

    let mesh = Mesh3d(meshes.add(Cuboid::new(40., 40., 40.)));
    let mat = MeshMaterial3d(materials.add(Color::srgb(0.4, 0.1, 0.4)));
    commands.spawn((mesh, mat, Transform::from_xyz(400., 20., -400.)));

    let mesh = Mesh3d(meshes.add(Cylinder::new(400., 40.)));
    let mat = MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.6)));
    commands.spawn((mesh, mat, Transform::from_xyz(100., 20., -100.)));

    let mesh = Mesh3d(meshes.add(Cylinder::new(60., 50.)));
    let mat = MeshMaterial3d(materials.add(Color::srgb(0.4, 0.5, 0.8)));
    commands.spawn((mesh, mat, Transform::from_xyz(-300., 25., 100.)));

    // Light
    commands.spawn((DirectionalLight::default(), Sun));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });

    let mut rng = rand::thread_rng();

    // setup point light grid
    for i in (-1000..1000).step_by(50) {
        for j in (-1000..1000).step_by(50) {
            commands.spawn((
                PointLight {
                    color: Color::srgb(
                        rng.gen_range(0.01..0.9),
                        rng.gen_range(0.01..0.9),
                        rng.gen_range(0.01..0.9),
                    ),
                    radius: 30.0,
                    range: 100.,
                    ..default()
                },
                Transform::from_xyz(i as f32, 4.0, j as f32),
            ));
        }
    }
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
