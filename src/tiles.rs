

use crate::tiles_config::ClayTilesConfig;
use bevy::prelude::*;

use bevy::asset::{AssetPath, LoadState};
use bevy::render::render_resource::{
    AddressMode, FilterMode, SamplerDescriptor, TextureDescriptor, TextureFormat,
};
use bevy::render::texture::{
    ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor,
};


#[derive(Component, Default)]
pub struct ClayTilesRoot {
    
    //pub tiles_data_loaded: bool,

    texture_image_handle: Option<Handle<Image>>,
    normal_image_handle: Option<Handle<Image>>,

    texture_image_finalized: bool, //need this for now bc of the weird way we have to load an array texture w polling and stuff... GET RID of me ???replace w enum ?
    normal_image_finalized: bool,
}

impl ClayTilesRoot {
    pub fn new() -> Self {
        ClayTilesRoot::default()
    }

      pub fn get_diffuse_texture_image(&self) -> &Option<Handle<Image>> {
        &self.texture_image_handle
    }

      pub fn get_normal_texture_image(&self) -> &Option<Handle<Image>> {
        &self.normal_image_handle
    }
}



pub fn load_tiles_texture_from_image(
    mut tile_root_query: Query<(&mut ClayTilesRoot, &ClayTilesConfig)>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    //  materials: Res <Assets<TerrainMaterialExtension>>,
) {
    for (mut tiles_data, tiles_config) in tile_root_query.iter_mut() {
        if tiles_data.texture_image_handle.is_none() {
            let array_texture_path = &tiles_config.diffuse_folder_path;

            let tex_image = asset_server.load(AssetPath::from_path(array_texture_path));
            tiles_data.texture_image_handle = Some(tex_image);
        }

        //try to load the height map data from the height_map_image_handle
        if !tiles_data.texture_image_finalized {
            let texture_image: &mut Image = match &tiles_data.texture_image_handle {
                Some(texture_image_handle) => {
                    let texture_image_loaded = asset_server.get_load_state(texture_image_handle);

                    if texture_image_loaded != Some(LoadState::Loaded) {
                        println!("tiles texture not yet loaded");
                        continue;
                    }

                    images.get_mut(texture_image_handle).unwrap()
                }
                None => continue,
            };

            //https://github.com/bevyengine/bevy/pull/10254
            texture_image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                label: None,
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                address_mode_w: ImageAddressMode::Repeat,
                mag_filter: ImageFilterMode::Linear,
                min_filter: ImageFilterMode::Linear,
                mipmap_filter: ImageFilterMode::Linear,
                ..default()
            });

            // Create a new array texture asset from the loaded texture.
            let desired_array_layers = tiles_config.texture_image_sections;

            let need_to_reinterpret = desired_array_layers > 1
                && texture_image.texture_descriptor.size.depth_or_array_layers == 1;

            if need_to_reinterpret {
                //info!("texture info {:?}" , texture_image.texture_descriptor.dimension, texture_image.size().depth_or_array_layers);

                texture_image.reinterpret_stacked_2d_as_array(desired_array_layers);
            }

           
            tiles_data.texture_image_finalized = true;
        }
    }
}

