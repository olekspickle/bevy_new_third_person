use crate::GameState;
use bevy::prelude::*;

pub struct ScenePlugin;

/// This plugin handles player related stuff like movement
/// Scene logic is only active during the State `GameState::Playing`
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Scene;

impl Scene {
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // circular base
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(4.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ));
        // cube
        let mesh_material3d: MeshMaterial3d<StandardMaterial> = MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255)));
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            mesh_material3d,

            Transform::from_xyz(0.0, 0.5, 0.0),
        ));
        // light
        commands.spawn((
            PointLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0),
        ));
        // camera
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }

    // This system logs all Mesh3d components in our world. Try making a change to a ComponentA in
    // load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
    // the changes appear in the console whenever you make a change.
    fn log_system(query: Query<(Entity, &Mesh3d), Changed<Mesh3d>>, res: Option<Res<MeshMaterial3d<StandardMaterial>>) {
        for (entity, mesh) in &query {
            info!("  Entity({})", entity.index());
            info!(
                "    Mesh: {{ x: {} y: {} }}\n",
                mesh.x, mesh.y
            );
        }
        if let Some(res) = res {
            if res.is_added() {
                info!("  New ResourceA: {{ score: {} }}\n", res.score);
            }
        }
    }
}
