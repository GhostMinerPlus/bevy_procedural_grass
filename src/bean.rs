use bevy::render::render_resource::Buffer;

mod chunk;
mod mesh;
mod wind;

pub use chunk::*;
pub use mesh::*;
pub use wind::*;

pub struct GrassChunkBuffer {
    pub buffer: Buffer,
    pub length: usize,
}
