use std::collections::HashMap;

use bevy::{
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::ExtractComponent, mesh::VertexAttributeValues, view::NoFrustumCulling,
    },
};
use bytemuck::{Pod, Zeroable};
use rand::Rng;

use crate::{
    assets::GrassData,
    bean::{CullDimension, GrassRenderInfo, Wind},
    res::GrassConfig,
};

pub use crate::plugin::render_com::*;

#[derive(Component, Deref, Clone, Asset, TypePath)]
pub struct GrassChunkData(pub Vec<GrassData>);

#[derive(Component, Resource, Default, Clone)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(Resource, InspectorOptions))]
pub struct GrassWind {
    pub wind_data: Wind,
    pub wind_map: Handle<Image>,
}

#[derive(Component, Clone)]
pub struct GrassChunks {
    pub chunk_size: f32,
    pub cull_dimension: CullDimension,
    pub chunks: HashMap<(i32, i32, i32), GrassChunkData>,
    pub loaded: HashMap<(i32, i32, i32), Handle<GrassChunkData>>,
    pub render: Vec<GrassRenderInfo>,
}

impl Default for GrassChunks {
    fn default() -> Self {
        Self {
            chunk_size: 30.,
            cull_dimension: CullDimension::D2,
            chunks: HashMap::new(),
            loaded: HashMap::new(),
            render: Vec::new(),
        }
    }
}

impl ExtractComponent for GrassChunks {
    type QueryData = &'static GrassChunks;
    type QueryFilter = ();
    type Out = RenderGrassChunks;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        Some(RenderGrassChunks(item.render.clone()))
    }
}

#[derive(Component, Default, Clone)]
pub struct RenderGrassChunks(pub Vec<GrassRenderInfo>);

#[derive(Component, Clone, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
#[repr(C)]
pub struct Blade {
    pub length: f32,
    pub width: f32,
    pub tilt: f32,
    pub tilt_variance: f32,
    pub p1_flexibility: f32,
    pub p2_flexibility: f32,
    pub curve: f32,
    pub specular: f32,
}

impl Default for Blade {
    fn default() -> Self {
        Self {
            length: 1.5,
            width: 0.05,
            tilt: 0.5,
            tilt_variance: 0.2,
            p1_flexibility: 0.5,
            p2_flexibility: 0.5,
            curve: 15.,
            specular: 0.02,
        }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub struct Grass {
    pub entity: Option<Entity>,
    pub density: u32,
    pub color: GrassColor,
    pub blade: Blade,
}

impl Default for Grass {
    fn default() -> Self {
        Self {
            density: 25,
            entity: None,
            color: GrassColor::default(),
            blade: Blade::default(),
        }
    }
}

impl Grass {
    pub(crate) fn generate_grass(
        &self,
        transform: &Transform,
        mesh: &Mesh,
        chunk_size: f32,
        asset_server: &AssetServer,
        config: &GrassConfig,
    ) -> HashMap<(i32, i32, i32), GrassChunkData> {
        let mut chunks: HashMap<(i32, i32, i32), GrassChunkData> = HashMap::new();

        if let Some(VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            if let Some(indices) = mesh.indices() {
                let mut triangle = Vec::new();
                for index in indices.iter() {
                    triangle.push(index);
                    if triangle.len() == 3 {
                        let _result: Vec<GrassData> = {
                            let v0 = Vec3::from(positions[triangle[0] as usize]) * transform.scale;
                            let v1 = Vec3::from(positions[triangle[1] as usize]) * transform.scale;
                            let v2 = Vec3::from(positions[triangle[2] as usize]) * transform.scale;

                            let normal = (v1 - v0).cross(v2 - v0).normalize();

                            let area = ((v1 - v0).cross(v2 - v0)).length() / 2.0;

                            let scaled_density = (self.density as f32 * area).ceil() as u32;

                            (0..scaled_density)
                                .filter_map(|_| {
                                    let mut rng = rand::thread_rng();

                                    let r1 = rng.gen::<f32>().sqrt();
                                    let r2 = rng.gen::<f32>();
                                    let barycentric = Vec3::new(1.0 - r1, r1 * (1.0 - r2), r1 * r2);

                                    let position = (v0 * barycentric.x
                                        + v1 * barycentric.y
                                        + v2 * barycentric.z)
                                        + transform.translation;

                                    let chunk_coords = (
                                        (position.x / chunk_size).floor() as i32,
                                        (position.y / chunk_size).floor() as i32,
                                        (position.z / chunk_size).floor() as i32,
                                    );

                                    let chunk_base = Vec3::new(
                                        chunk_coords.0 as f32,
                                        chunk_coords.1 as f32,
                                        chunk_coords.2 as f32,
                                    ) * chunk_size;
                                    let chunk_pos = position - chunk_base;
                                    let chunk_uvw = Vec3::new(
                                        chunk_pos.x / chunk_size,
                                        chunk_pos.y / chunk_size,
                                        chunk_pos.z / chunk_size,
                                    );

                                    let instance = GrassData {
                                        position,
                                        normal,
                                        chunk_uvw,
                                    };

                                    chunks
                                        .entry(chunk_coords)
                                        .or_insert_with(|| GrassChunkData(Vec::new()))
                                        .0
                                        .push(instance);

                                    None
                                })
                                .collect::<Vec<_>>()
                        };
                        triangle.clear();
                    }
                }
            }
        }

        chunks
    }
}

impl ExtractComponent for Grass {
    type QueryData = &'static Grass;
    type QueryFilter = ();
    type Out = (GrassColor, Blade);

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        Some((item.color.clone(), item.blade.clone()))
    }
}

#[derive(Component, Clone, Copy)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub struct GrassColor {
    pub ao: Color,
    pub color_1: Color,
    pub color_2: Color,
}

impl GrassColor {
    pub fn to_array(&self) -> [[f32; 4]; 3] {
        [
            self.ao.as_linear_rgba_f32(),
            self.color_1.as_linear_rgba_f32(),
            self.color_2.as_linear_rgba_f32(),
        ]
    }
}

impl Default for GrassColor {
    fn default() -> Self {
        Self {
            ao: Color::rgba_from_array([0.01, 0.02, 0.05, 1.0]),
            color_1: Color::rgba_from_array([0.1, 0.23, 0.09, 1.0]),
            color_2: Color::rgba_from_array([0.12, 0.39, 0.15, 1.0]),
        }
    }
}

#[derive(Bundle, Default)]
pub struct GrassBundle {
    pub mesh: Handle<Mesh>,
    pub lod: GrassLODMesh,
    pub grass: Grass,
    pub grass_chunks: GrassChunks,
    #[bundle()]
    pub spatial: SpatialBundle,
    pub frustum_culling: NoFrustumCulling,
}

#[derive(Component, Default, Clone)]
pub struct GrassLODMesh {
    pub mesh_handle: Option<Handle<Mesh>>,
}

impl GrassLODMesh {
    pub fn new(mesh_handle: Handle<Mesh>) -> Self {
        Self {
            mesh_handle: Some(mesh_handle),
        }
    }
}

impl ExtractComponent for GrassLODMesh {
    type QueryData = &'static GrassLODMesh;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        Some(item.clone())
    }
}
