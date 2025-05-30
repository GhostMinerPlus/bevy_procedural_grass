use bevy::{
    ecs::{
        query::ROQueryItem,
        system::{
            lifetimeless::{Read, SQuery, SRes},
            SystemParamItem,
        },
    },
    pbr::{RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup},
    render::{
        mesh::{GpuBufferInfo, GpuMesh},
        render_asset::RenderAssets,
        render_phase::{
            PhaseItem, RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
        },
        render_resource::BindGroup,
    },
};

use crate::{
    bean::{GrassChunkBuffer, GrassLOD},
    com::{BufferBindGroup, Grass, GrassLODMesh, GrassWind, RenderGrassChunks},
};

pub type DrawGrass = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetGrassBindGroup<2>,
    SetWindBindGroup<3>,
    DrawGrassInstanced,
);

pub struct SetGrassBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetGrassBindGroup<I> {
    type Param = SQuery<Read<BufferBindGroup<Grass>>>;
    type ViewQuery = ();
    type ItemQuery = ();

    fn render<'w>(
        item: &P,
        _view: (),
        _entity: Option<ROQueryItem<'w, Self::ItemQuery>>,
        bind_groups: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(bind_group) = bind_groups.get(item.entity()).ok() else {
            log::warn!("Grass bind group not found for entity: {:?}", item.entity());
            return RenderCommandResult::Failure;
        };

        let bind_group = unsafe { &*(&bind_group.bind_group as *const BindGroup) };

        pass.set_bind_group(I, bind_group, &[]);
        RenderCommandResult::Success
    }
}

pub struct SetWindBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetWindBindGroup<I> {
    type Param = SRes<BufferBindGroup<GrassWind>>;
    type ViewQuery = ();
    type ItemQuery = Read<BufferBindGroup<GrassWind>>;

    fn render<'w>(
        _item: &P,
        _view: (),
        local_wind: Option<ROQueryItem<'w, Self::ItemQuery>>,
        wind_bind_group: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let bind_group = if let Some(local_wind) = local_wind {
            local_wind
        } else {
            wind_bind_group.into_inner()
        };
        pass.set_bind_group(I, &bind_group.bind_group, &[]);
        RenderCommandResult::Success
    }
}

pub struct DrawGrassInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawGrassInstanced {
    type Param = (
        SRes<RenderAssets<GpuMesh>>,
        SRes<RenderMeshInstances>,
        SRes<RenderAssets<GrassChunkBuffer>>,
        SQuery<(Read<GrassLODMesh>, Read<RenderGrassChunks>)>,
    );
    type ViewQuery = ();
    type ItemQuery = ();

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        _entity: Option<ROQueryItem<'w, Self::ItemQuery>>,
        (meshes, render_mesh_instances, grass_data, entity): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(item.entity())
        else {
            return RenderCommandResult::Failure;
        };

        let meshes = meshes.into_inner();

        let gpu_mesh_high = match meshes.get(mesh_instance.mesh_asset_id) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        let (lod, chunks) = entity.get(item.entity()).unwrap();

        let gpu_mesh_low = if let Some(lod) = &lod.mesh_handle {
            match meshes.get(lod) {
                Some(gpu_mesh) => gpu_mesh,
                None => return RenderCommandResult::Failure,
            }
        } else {
            gpu_mesh_high
        };

        let grass_data_inner = grass_data.into_inner();

        for (_, chunk) in chunks.0.iter().enumerate() {
            let gpu_grass = match grass_data_inner.get(&chunk.1) {
                Some(gpu_grass) => gpu_grass,
                None => return RenderCommandResult::Failure,
            };

            let gpu_mesh = match chunk.0 {
                GrassLOD::Low => &gpu_mesh_low,
                GrassLOD::High => &gpu_mesh_high,
            };

            pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, gpu_grass.buffer.slice(..));

            match &gpu_mesh.buffer_info {
                GpuBufferInfo::Indexed {
                    buffer,
                    index_format,
                    count,
                } => {
                    pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                    pass.draw_indexed(0..*count, 0, 0..gpu_grass.length as u32);
                }
                GpuBufferInfo::NonIndexed => {
                    pass.draw(0..gpu_mesh.vertex_count, 0..gpu_grass.length as u32);
                }
            }
        }

        RenderCommandResult::Success
    }
}
