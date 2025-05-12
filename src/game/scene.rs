use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    pbr::DirectionalLightShadowMap,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_fix_gltf_coordinate_system::prelude::*;

/// This plugin handles loading and saving scenes
/// Scene logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .add_plugins(FixGltfCoordinateSystemPlugin)
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_systems(OnEnter(Screen::Gameplay), setup);
}

pub(crate) fn setup(
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let main_plane = config.geometry.main_plane;

    // Plane
    let mesh = Mesh3d(meshes.add(Cuboid::new(main_plane, 0., main_plane)));
    let mat = MeshMaterial3d(materials.add(SAND_YELLOW));
    commands.spawn((
        mat,
        mesh,
        Transform::default(),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
    ));

    let size = main_plane / 2.0;
    let geom = config.geometry.clone();
    for i in 0..geom.quantity {
        let i = i as f32;
        let (low, upper) = (main_plane / 100.0, main_plane / 40.0);
        let step = (upper - low) / geom.quantity as f32;

        let y_size = low + step * i;
        let x_size = low + step * i;
        let (x, y, mut z) = (
            -size / 4.0 + i * x_size, // + step * 20.0,
            y_size / 2.0 + i * step,
            -size / 4.0,
        );
        let (mesh, color) = if i % 2.0 == 0.0 {
            (Mesh::from(Cuboid::new(x_size, y_size, x_size)), GREEN)
        } else {
            z += size / 2.0;
            (Mesh::from(Sphere::new(y_size)), LIGHT_BLUE)
        };
        let material = materials.add(StandardMaterial {
            base_color: color,
            #[cfg(feature = "enhanced")]
            specular_tint: Color::WHITE,
            ..default()
        });

        let mesh3d = Mesh3d(meshes.add(mesh.clone()));
        let mat = MeshMaterial3d(material.clone());
        let pos = Transform::from_xyz(x, y, z);
        commands.spawn((
            mat,
            pos,
            mesh3d,
            RigidBody::Static,
            Collider::trimesh_from_mesh(&mesh).expect("failed to create collider for mesh"),
        ));
    }

    // TODO: add spatial boombox object
    // // soundtrack boombox
    // commands.spawn((
    //     Boombox,
    //     Mesh3d(meshes.add(Sphere::new(0.2).mesh().uv(32, 18))),
    //     MeshMaterial3d(materials.add(LIGHT_BLUE)),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    // ));

    // to see something when suns go away
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        ..Default::default()
    });
}

/// Creates a colorful test pattern
pub(crate) fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
