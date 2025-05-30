use bevy::{asset::Asset, reflect::TypePath, render::render_resource::Buffer};

mod chunk;
mod mesh;
mod wind;

pub use chunk::*;
pub use mesh::*;
pub use wind::*;

#[derive(Asset, TypePath)]
pub struct GrassChunkBuffer {
    pub buffer: Buffer,
    pub length: usize,
}
