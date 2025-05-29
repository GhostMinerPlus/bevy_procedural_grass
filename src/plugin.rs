use bevy::{
    core_pipeline::core_3d::Opaque3d,
    prelude::*,
    render::{
        render_phase::AddRenderCommand, render_resource::SpecializedMeshPipelines, Render,
        RenderApp, RenderSet,
    },
};

use crate::{command::DrawGrass, pipeline::GrassPipeline};

pub mod render_com;
mod render_sys;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_render_command::<Opaque3d, DrawGrass>()
            .init_resource::<SpecializedMeshPipelines<GrassPipeline>>()
            .add_systems(
                Render,
                (
                    render_sys::grass_queue.in_set(RenderSet::QueueMeshes),
                    render_sys::prepare_grass_buffers.in_set(RenderSet::PrepareResources),
                    render_sys::prepare_global_wind_buffers.in_set(RenderSet::PrepareResources),
                    render_sys::prepare_local_wind_buffers.in_set(RenderSet::PrepareResources),
                    render_sys::prepare_grass_bind_group.in_set(RenderSet::PrepareBindGroups),
                    render_sys::prepare_global_wind_bind_group.in_set(RenderSet::PrepareBindGroups),
                    render_sys::prepare_local_wind_bind_group.in_set(RenderSet::PrepareBindGroups),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp).init_resource::<GrassPipeline>();
    }
}
