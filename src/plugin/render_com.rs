use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::render_resource::{BindGroup, Buffer},
};

#[derive(Component, Resource, Clone)]
pub struct BufferBindGroup<T> {
    pub bind_group: BindGroup,
    _marker: PhantomData<T>,
}

impl<T> BufferBindGroup<T> {
    pub fn new(bind_group: BindGroup) -> Self {
        Self {
            bind_group,
            _marker: PhantomData,
        }
    }
}

#[derive(Component, Clone)]
pub struct GrassBuffer {
    pub color_buffer: Buffer,
    pub blade_buffer: Buffer,
}

#[derive(Component, Resource, Clone)]
pub struct WindBuffer {
    pub(crate) buffer: Buffer,
}
