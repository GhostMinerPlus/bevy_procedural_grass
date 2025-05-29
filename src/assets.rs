use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
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
    type ExtractedAsset = GrassChunkData;
    type PreparedAsset = GrassChunkBuffer;
    type Param = SRes<RenderDevice>;

    fn extract_asset(&self) -> Self::ExtractedAsset {
        GrassChunkData(self.0.clone())
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let render_device = param;

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(extracted_asset.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });

        Ok(GrassChunkBuffer {
            buffer,
            length: extracted_asset.len(),
        })
    }
}
