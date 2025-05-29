use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin, extract_resource::ExtractResourcePlugin,
        render_asset::RenderAssetPlugin,
    },
};

mod assets;
mod command;
mod pipeline;
mod plugin;
mod sys;
mod util;

pub mod bean;
pub mod com;
pub mod res;

pub mod prelude {
    pub use crate::ProceduralGrassPlugin;
}

pub(crate) const GRASS_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(195_094_223_228_228_028_086_047_086_167_255_040_126);

#[derive(Default, Clone)]
pub struct ProceduralGrassPlugin {
    pub config: res::GrassConfig,
    pub wind: com::GrassWind,
}

impl Plugin for ProceduralGrassPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GRASS_SHADER_HANDLE,
            "shader/grass.wgsl",
            Shader::from_wgsl
        );

        #[cfg(feature = "bevy-inspector-egui")]
        {
            app.register_type::<Grass>()
                .register_type::<GrassWind>()
                .register_type::<GrassConfig>();
        }
        app.insert_resource(self.wind.clone())
            .insert_resource(self.config)
            .add_systems(Startup, sys::create_wind_map)
            .add_systems(PostStartup, sys::generate_grass)
            .add_systems(Update, sys::grass_culling)
            .init_asset::<com::GrassChunkData>()
            .add_plugins(RenderAssetPlugin::<com::GrassChunkData>::default())
            .add_plugins((
                ExtractComponentPlugin::<com::Grass>::default(),
                ExtractComponentPlugin::<com::GrassChunks>::default(),
                ExtractComponentPlugin::<com::GrassLODMesh>::default(),
                ExtractComponentPlugin::<com::GrassWind>::default(),
                ExtractResourcePlugin::<com::GrassWind>::default(),
                plugin::RenderPlugin,
            ));
    }
}
