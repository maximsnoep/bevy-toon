use bevy::asset;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

const UUID: &str = "1347c9b7-c46a-48e7-b7b8-023a354b7cac";

pub struct ToonPlugin;
impl Plugin for ToonPlugin {
    fn build(&self, app: &mut App) {
        asset::load_internal_asset!(
            app,
            asset::uuid_handle!(UUID),
            "./shader.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<ToonMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct ToonMaterial {
    #[uniform(0)]
    pub view_dir: Vec3,
}

impl Material for ToonMaterial {
    fn fragment_shader() -> ShaderRef {
        asset::uuid_handle!(UUID).into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
