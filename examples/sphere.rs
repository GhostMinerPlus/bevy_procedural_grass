use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_flycam::PlayerPlugin;
use bevy_procedural_grass::{
    bean::GrassMesh,
    com::{Grass, GrassChunks, GrassColor},
    prelude::*,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        PlayerPlugin,
        ProceduralGrassPlugin::default(), // add procedural grass plugin
    ))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let terrain_mesh = Sphere::new(1.0);

    let terrain = commands
        .spawn((
            Mesh3d(meshes.add(terrain_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.05, 0.0),
                reflectance: 0.0,
                ..Default::default()
            })),
            Transform::from_scale(Vec3::new(20.0, 20.0, 20.0)),
        ))
        .id();

    // add grass bundle
    commands.spawn((
        Mesh3d(meshes.add(GrassMesh::mesh(7))),
        Grass {
            density: 25,
            entity: Some(terrain.clone()), // set entity grass will be placed on (must have a mesh and transform)
            ..default()
        },
        NoFrustumCulling::default(),
        Transform::IDENTITY,
        GrassChunks::default(),
        GrassColor::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform::from_rotation(Quat::from_xyzw(
            -0.4207355, -0.4207355, 0.22984886, 0.77015114,
        )),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.75, 4.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        // GrassDisplacer {
        //     width: 15.,
        //     base_offset: Vec3::new(0., -2., 0.),
        // },
    ));
}
