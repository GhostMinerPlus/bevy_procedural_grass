use bevy::{
    core_pipeline::core_3d::Transparent3d,
    prelude::*,
    render::{
        Render, RenderApp, RenderSet, render_phase::AddRenderCommand,
        render_resource::SpecializedMeshPipelines,
    },
};

use crate::{command::DrawCustom, pipeline::CustomPipeline};

mod render_sys;

pub(super) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
            .add_systems(
                Render,
                (
                    render_sys::queue_custom.in_set(RenderSet::QueueMeshes),
                    render_sys::prepare_instance_buffers.in_set(RenderSet::PrepareResources),
                ),
            );
    }
}
