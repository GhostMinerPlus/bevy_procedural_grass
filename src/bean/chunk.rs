use bevy::prelude::*;

use crate::com::GrassChunkData;

#[derive(Clone, Copy)]
pub enum GrassLOD {
    High,
    Low,
}

#[derive(Clone, Copy)]
pub enum CullDimension {
    D2,
    D3,
}

impl Default for CullDimension {
    fn default() -> Self {
        Self::D2
    }
}

pub type GrassRenderInfo = (GrassLOD, Handle<GrassChunkData>);
