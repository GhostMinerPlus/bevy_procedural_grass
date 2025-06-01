use bevy::{
    asset::{load_internal_asset, weak_handle},
    prelude::*,
    render::{RenderApp, extract_component::ExtractComponentPlugin},
};

mod command;
mod pipeline;
mod plugin;
mod sys;

pub mod bean;
pub mod com;

/// This example uses a shader source file from the assets subdirectory
const INSTANCE_SHADER_HANDLE: Handle<Shader> = weak_handle!("c42a18ed-35fd-436b-bed3-d3d19f67b7cd");

pub struct InstancePlugin;

impl Plugin for InstancePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            INSTANCE_SHADER_HANDLE,
            "shaders/instance.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins((
            ExtractComponentPlugin::<com::InstanceMaterialData>::default(),
            plugin::RenderPlugin,
        ));
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<pipeline::CustomPipeline>();
    }
}
