use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{
        mesh::{PlaneMeshBuilder, VertexAttributeValues},
        view::NoIndirectDrawing,
    },
};
use bevy_procedural_grass::{
    bean::{GrassMesh, Wind},
    com::{Grass, GrassLODMesh, GrassWind},
    prelude::*,
    res::GrassConfig,
};

use noise::NoiseFn;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "info,bevy_procedural_grass=debug".into(),
                ..Default::default()
            }),
            ProceduralGrassPlugin {
                config: GrassConfig::default(),
                wind: GrassWind {
                    wind_data: Wind {
                        speed: 0.1,
                        amplitude: 4.,
                        ..default()
                    },
                    ..default()
                },
            },
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut terrain_mesh = PlaneMeshBuilder::new(Dir3::Y, Vec2::splat(100.0))
        .subdivisions(100)
        .build();
    if let Some(positions) = terrain_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        if let VertexAttributeValues::Float32x3(positions) = positions {
            for position in positions.iter_mut() {
                let y = noise::Perlin::new(1)
                    .get([((position[0]) * 0.05) as f64, ((position[2]) * 0.05) as f64])
                    as f32;
                position[1] += y;
            }
        }
    }

    let terrain = commands
        .spawn((
            Mesh3d(meshes.add(terrain_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.05, 0.0),
                reflectance: 0.0,
                ..default()
            })),
            Transform::from_scale(Vec3::new(1.0, 3.0, 1.0)),
        ))
        .id();

    commands.spawn((
        Grass {
            entity: Some(terrain.clone()),
            ..default()
        },
        Mesh3d(meshes.add(GrassMesh::mesh(7))),
        GrassLODMesh::new(meshes.add(GrassMesh::mesh(3))),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.75, 4.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_xyzw(
            -0.4207355, -0.4207355, 0.22984886, 0.77015114,
        )),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::new(2.5, 3.5, 0.0), Vec3::Y),
        NoIndirectDrawing,
    ));
}
