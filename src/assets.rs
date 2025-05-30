use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssetUsages},
        render_resource::{BufferInitDescriptor, BufferUsages},
        renderer::RenderDevice,
    },
};
use bytemuck::{Pod, Zeroable};

use crate::{bean::GrassChunkBuffer, com::GrassChunkData};

#[derive(Clone, Copy, Pod, Zeroable, Reflect, Debug)]
#[repr(C)]
pub struct GrassData {
    pub position: Vec3,
    pub normal: Vec3,
    pub chunk_uvw: Vec3,
}

impl Default for GrassChunkData {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl RenderAsset for GrassChunkData {
    type PreparedAsset = GrassChunkBuffer;
    type Param = SRes<RenderDevice>;

    fn asset_usage(&self) -> RenderAssetUsages {
        RenderAssetUsages::all()
    }

    fn prepare_asset(
        self,
        param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self>> {
        let render_device = param;

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(self.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });

        Ok(GrassChunkBuffer {
            buffer,
            length: self.len(),
        })
    }
}
