use bevy::asset;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

// The shader (WESL), embedded in the plugin.
const SHADER: &str = "embedded://bevy_toon/shader.wesl";

pub struct ToonPlugin;
impl Plugin for ToonPlugin {
    fn build(&self, app: &mut App) {
        asset::embedded_asset!(app, "shader.wesl");
        app.add_plugins(MaterialPlugin::<ToonMaterial>::default());
    }
}

/// The toon material: lit from the camera, in the vertex colors of the mesh.
#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct ToonMaterial {}

impl Material for ToonMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
