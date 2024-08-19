use bevy::asset::VisitAssetDependencies;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use bevy::render::render_asset::RenderAssets;

use bevy::pbr::ExtendedMaterial;
use bevy::pbr::StandardMaterialFlags;
use bevy::pbr::StandardMaterialUniform;

use bevy::pbr::MaterialExtension;


pub type TileMaterialExtension = ExtendedMaterial<StandardMaterial, TileMaterial>;


pub const TILE_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(2155284082028047579);

/*
#[derive(Clone, ShaderType, Default, Debug)]
pub struct ChunkMaterialUniforms {
    pub color_texture_expansion_factor: f32,
    pub chunk_uv: Vec4, //start_x, start_y, end_x, end_y   -- used to subselect a region from the splat texture
}

#[derive(Clone, ShaderType, Default, Debug)]
pub struct ToolPreviewUniforms {
    pub tool_coordinates: Vec2,
    pub tool_radius: f32,
    pub tool_color: Vec3,
}*/

#[derive(Asset, AsBindGroup, TypePath, Clone, Debug, Default)]
pub struct TileMaterial {
      #[uniform(20)]
     pub color_texture_expansion_factor: f32,

      #[uniform(21)]
     pub diffuse_color_tint: Vec4,

       #[uniform(22)]
     pub tile_texture_index: u32,

     

    /*#[uniform(20)]
    pub chunk_uniforms: ChunkMaterialUniforms,

    #[uniform(21)]
    pub tool_preview_uniforms: ToolPreviewUniforms,
	*/
    #[texture(24, dimension = "2d_array")]
    #[sampler(25)]
    pub diffuse_texture: Option<Handle<Image>>,

    #[texture(26, dimension = "2d_array")]
    #[sampler(27)]
    pub normal_texture: Option<Handle<Image>>,

     
}

impl MaterialExtension for TileMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(TILE_SHADER_HANDLE)
    }

    fn deferred_fragment_shader() -> ShaderRef {
        ShaderRef::Handle(TILE_SHADER_HANDLE)
    }
}
