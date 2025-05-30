use bevy::{
    core_pipeline::core_3d::{Opaque3d, Opaque3dBinKey},
    pbr::{MeshPipelineKey, RenderMeshInstances},
    prelude::*,
    render::{
        mesh::GpuMesh,
        render_asset::RenderAssets,
        render_phase::{
            BinnedPhaseItem, BinnedRenderPhaseType, DrawFunctions, PhaseItemExtraIndex,
            ViewBinnedRenderPhases,
        },
        render_resource::{
            BindGroupEntries, BindingResource, BufferBinding, BufferInitDescriptor, BufferUsages,
            PipelineCache, SpecializedMeshPipelines,
        },
        renderer::RenderDevice,
        texture::{FallbackImage, GpuImage},
        view::ExtractedView,
    },
};

use crate::{
    com::{Blade, Grass, GrassColor, GrassWind, RenderGrassChunks},
    command::DrawGrass,
    pipeline::GrassPipeline,
};

use super::render_com::{BufferBindGroup, GrassBuffer, WindBuffer};

pub(super) fn grass_queue(
    opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
    custom_pipeline: Res<GrassPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<GrassPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<GpuMesh>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    material_meshes: Query<Entity, With<RenderGrassChunks>>,
    views: Query<(Entity, &ExtractedView)>,
    mut phases: ResMut<ViewBinnedRenderPhases<Opaque3d>>,
) {
    let draw_custom = opaque_3d_draw_functions.read().id::<DrawGrass>();

    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());
    for (view_entity, view) in &views {
        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        if let Some(opaque_phase) = phases.get_mut(&view_entity) {
            for entity in &material_meshes {
                let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(entity)
                else {
                    continue;
                };
                let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                    continue;
                };
                let key =
                    view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology());
                let pipeline = pipelines
                    .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
                    .unwrap();

                opaque_phase.add(
                    Opaque3dBinKey {
                        pipeline,
                        draw_function: draw_custom,
                        asset_id: mesh_instance.mesh_asset_id.into(),
                        material_bind_group_id: None,
                        lightmap_image: None,
                    },
                    entity,
                    BinnedRenderPhaseType::BatchableMesh,
                );
            }
        }
    }
}

pub(super) fn prepare_grass_buffers(
    mut commands: Commands,
    query: Query<(Entity, &GrassColor, &Blade)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, color, blade) in &query {
        let color_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("color buffer"),
            contents: bytemuck::cast_slice(&color.to_array()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let blade_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("blade buffer"),
            contents: bytemuck::cast_slice(&[blade.clone()]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        commands.entity(entity).insert(GrassBuffer {
            color_buffer,
            blade_buffer,
        });
    }
}

pub(super) fn prepare_global_wind_buffers(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    wind: Res<GrassWind>,
) {
    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("wind buffer"),
        contents: bytemuck::cast_slice(&[wind.wind_data.clone()]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    commands.insert_resource(WindBuffer { buffer });
}

pub(super) fn prepare_local_wind_buffers(
    mut commands: Commands,
    query: Query<(Entity, &GrassWind)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, grass_wind) in &query {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("local wind buffer"),
            contents: bytemuck::cast_slice(&[grass_wind.wind_data.clone()]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        commands.entity(entity).insert(WindBuffer { buffer });
    }
}

pub(super) fn prepare_grass_bind_group(
    mut commands: Commands,
    pipeline: Res<GrassPipeline>,
    render_device: Res<RenderDevice>,
    query: Query<(Entity, &GrassBuffer)>,
) {
    let layout = pipeline.grass_layout.clone();

    for (entity, grass) in query.iter() {
        let bind_group = render_device.create_bind_group(
            Some("grass bind group"),
            &layout,
            &BindGroupEntries::sequential((
                BufferBinding {
                    buffer: &grass.color_buffer,
                    offset: 0,
                    size: None,
                },
                BufferBinding {
                    buffer: &grass.blade_buffer,
                    offset: 0,
                    size: None,
                },
            )),
        );

        commands
            .entity(entity)
            .insert(BufferBindGroup::<Grass>::new(bind_group));
    }
}

pub(super) fn prepare_global_wind_bind_group(
    mut commands: Commands,
    pipeline: Res<GrassPipeline>,
    render_device: Res<RenderDevice>,
    wind: Res<GrassWind>,
    wind_buffer: Res<WindBuffer>,
    fallback_img: Res<FallbackImage>,
    images: Res<RenderAssets<GpuImage>>,
) {
    let layout = pipeline.wind_layout.clone();

    let wind_map_texture = if let Some(texture) = images.get(&wind.wind_map) {
        &texture.texture_view
    } else {
        &fallback_img.d2.texture_view
    };

    let bind_group = render_device.create_bind_group(
        Some("wind bind group"),
        &layout,
        &BindGroupEntries::sequential((
            BufferBinding {
                buffer: &wind_buffer.buffer,
                offset: 0,
                size: None,
            },
            BindingResource::TextureView(&wind_map_texture),
        )),
    );

    commands.insert_resource(BufferBindGroup::<GrassWind>::new(bind_group));
}

pub(super) fn prepare_local_wind_bind_group(
    mut commands: Commands,
    pipeline: Res<GrassPipeline>,
    render_device: Res<RenderDevice>,
    query: Query<(Entity, &GrassWind, &WindBuffer)>,
    fallback_img: Res<FallbackImage>,
    images: Res<RenderAssets<GpuImage>>,
) {
    let layout = pipeline.wind_layout.clone();

    for (entity, grass_wind, wind_buffer) in query.iter() {
        let wind_map_texture = if let Some(texture) = images.get(&grass_wind.wind_map) {
            &texture.texture_view
        } else {
            &fallback_img.d2.texture_view
        };

        let bind_group = render_device.create_bind_group(
            Some("local wind bind group"),
            &layout,
            &BindGroupEntries::sequential((
                BufferBinding {
                    buffer: &wind_buffer.buffer,
                    offset: 0,
                    size: None,
                },
                BindingResource::TextureView(&wind_map_texture),
            )),
        );

        commands
            .entity(entity)
            .insert(BufferBindGroup::<GrassWind>::new(bind_group));
    }
}
