use bevy::{
    pbr::{MeshPipeline, MeshPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            BindGroupLayout, BindGroupLayoutEntry, BindingType, BufferBindingType,
            RenderPipelineDescriptor, ShaderStages, SpecializedMeshPipeline,
            SpecializedMeshPipelineError, TextureSampleType, TextureViewDimension, VertexAttribute,
            VertexBufferLayout, VertexFormat, VertexStepMode,
        },
        renderer::RenderDevice,
    },
};

use crate::{assets::GrassData, GRASS_SHADER_HANDLE};

#[derive(Resource)]
pub struct GrassPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    pub grass_layout: BindGroupLayout,
    pub wind_layout: BindGroupLayout,
}

impl FromWorld for GrassPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let mesh_pipeline = world.resource::<MeshPipeline>();

        let grass_layout = render_device.create_bind_group_layout(
            "grass_layout",
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        );

        let wind_layout = render_device.create_bind_group_layout(
            "wind_layout",
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        );

        GrassPipeline {
            shader: GRASS_SHADER_HANDLE,
            mesh_pipeline: mesh_pipeline.clone(),
            grass_layout,
            wind_layout,
        }
    }
}

impl SpecializedMeshPipeline for GrassPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        descriptor
            .vertex
            .shader_defs
            .push("MESH_BINDGROUP_1".into());

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<GrassData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: VertexFormat::Float32x3.size(),
                    shader_location: 4,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: VertexFormat::Float32x3.size() + VertexFormat::Float32x3.size(),
                    shader_location: 5,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        // layout
        descriptor.layout.push(self.grass_layout.clone());
        descriptor.layout.push(self.wind_layout.clone());
        //
        descriptor.primitive.cull_mode = None;
        Ok(descriptor)
    }
}
