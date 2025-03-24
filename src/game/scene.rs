use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use rand::{Rng, thread_rng};

const SUN: Color = Color::srgb(248.0 / 255.0, 176.0 / 255.0, 14.0 / 255.0);

/// This plugin handles loading and saving scenes
/// Scene logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_systems(OnEnter(Screen::Gameplay), setup);
}

pub fn setup(
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let main_plane = config.geometry.main_plane;
    // Plane
    let mesh = Mesh3d(meshes.add(Cuboid::new(main_plane, 0., main_plane)));
    let green = MeshMaterial3d(materials.add(Color::srgb(0.3, 0.9, 0.3)));
    commands.spawn((
        mesh,
        green,
        Transform::default(),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
    ));

    let mut rng = thread_rng();
    let geom = config.geometry.clone();
    for i in 0..geom.quantity {
        let (low, upper) = (main_plane / 100.0, main_plane / 20.0);
        let height = rng.gen_range(low..upper);
        let x_width = rng.gen_range(low..upper);
        let z_width = rng.gen_range(low..upper);
        let mesh = if i % 2 == 0 {
            Mesh3d(meshes.add(Cuboid::new(x_width, height, z_width)))
        } else {
            Mesh3d(meshes.add(Cylinder::new(x_width, height)))
        };
        let (r, g, b) = (
            rng.gen_range(0.01..0.99),
            rng.gen_range(0.01..0.99),
            rng.gen_range(0.01..0.99),
        );
        let mat = MeshMaterial3d(materials.add(Color::srgb(r, g, b)));

        let half_plane = main_plane / 2.0;
        let (x, y, z) = (
            rng.gen_range((-half_plane + x_width)..(half_plane - x_width)),
            // this will send them flying!
            rng.gen_range(height / 3.0..height / 1.5),
            rng.gen_range((-half_plane + x_width)..(half_plane - x_width)),
        );
        commands.spawn((
            mesh,
            mat,
            RigidBody::Static,
            Collider::capsule(x_width.max(z_width) / 2.0, height),
            Transform::from_xyz(x, y, z),
        ));
    }

    // Light
    commands.spawn((
        DirectionalLight {
            color: SUN,
            //shadows_enabled: true,
            ..Default::default()
        },
        Sun,
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    //// setup point light grid
    //for i in (-PLANE_WIDTH_INT..PLANE_WIDTH_INT).step_by(50) {
    //    for j in (-PLANE_WIDTH_INT..PLANE_WIDTH_INT).step_by(50) {
    //        commands.spawn((
    //            PointLight {
    //                color: Color::srgb(
    //                    rng.gen_range(0.01..0.9),
    //                    rng.gen_range(0.01..0.9),
    //                    rng.gen_range(0.01..0.9),
    //                ),
    //                radius: 30.0,
    //                range: 100.,
    //                ..default()
    //            },
    //            Transform::from_xyz(i as f32, 20.0, j as f32),
    //        ));
    //    }
    //}
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
