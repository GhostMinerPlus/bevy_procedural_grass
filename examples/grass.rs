use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_flycam::PlayerPlugin;
use bevy_procedural_grass::{
    bean::GrassMesh,
    com::{Grass, GrassChunks, GrassColor, GrassLODMesh},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            ProceduralGrassPlugin::default(), // add grass plugin
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let terrain = commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.05, 0.0),
                reflectance: 0.0,

                ..default()
            })),
            Transform::from_scale(Vec3::new(100.0, 3.0, 100.0)),
        ))
        .id();

    // spawn grass
    commands.spawn((
        Mesh3d(meshes.add(GrassMesh::mesh(7))),
        Grass {
            entity: Some(terrain.clone()), // set entity that grass will generate on top of.
            ..default()
        },
        GrassLODMesh::new(meshes.add(GrassMesh::mesh(3))), // optional: enables LOD
        NoFrustumCulling::default(),
        Transform::IDENTITY,
        GrassChunks::default(),
        GrassColor::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)).with_scale(Vec3::new(1.0, 5.0, 1.0)),
        // GrassDisplacer {
        //     width: 15.,
        //     base_offset: Vec3::new(0.0, -2.0, 0.0),
        // },
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_xyzw(
            -0.4207355, -0.4207355, 0.22984886, 0.77015114,
        )),
        ..default()
    });
}
