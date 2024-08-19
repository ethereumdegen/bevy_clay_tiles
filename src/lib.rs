
use crate::tile_types_config::TileTypesConfig;
use crate::tiles::ClayTilesTypesConfigResource;
use crate::tile_gizmos::tile_gizmos_plugin;
use transform_gizmo_bevy::TransformGizmoPlugin;
use crate::tiles::ClayTilesConfigResource;

use bevy::asset::{AssetPath, LoadState};
use crate::tiles_config::ClayTilesConfig;
use crate::tiles::ClayTilesTexturingResource;
use bevy_mod_raycast::prelude::CursorRayPlugin;
use crate::tile_edit::tile_edit_plugin;

use crate::clay_tile_block::clay_tile_block_plugin;
use crate::tiles::load_tiles_texture_from_image;
use crate::tile_material::TileMaterialExtension;
use crate::tile_material::TILE_SHADER_HANDLE;
//use crate::tile_edit::tile_edit_plugin; 
use bevy::{asset::load_internal_asset, prelude::*};
 

 
 pub mod tiles;
 
 pub mod clay_tile_block; 
 pub mod tile_material;
 pub mod tiles_config;
 pub mod tile_types_config;
 pub mod tile_edit;
 pub mod pre_mesh;
 pub mod clay_tile;
 pub mod tile_gizmos;
 

pub struct BevyClayTilesPlugin {
    pub config: ClayTilesConfig
}
 
impl Plugin for BevyClayTilesPlugin {
    fn build(&self, app: &mut App) {

        app.init_resource::<ClayTilesTexturingResource>();
        app.insert_resource(ClayTilesTypesConfigResource::new( 
            &TileTypesConfig::load_from_file(
              &format!("assets/{}",AssetPath::from_path( self.config.get_tile_types_config_path() ).path().to_str().unwrap() )  
            ).unwrap()
        
            )  ); 
        app.insert_resource(ClayTilesConfigResource(self.config.clone())) ;
        //app.init_resource::<ClayTilesConfigResource>();

        load_internal_asset!(
            app,
            TILE_SHADER_HANDLE,
            "shaders/tile.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<TileMaterialExtension>::default());
       
       if !app.is_plugin_added::<CursorRayPlugin>() {
             app.add_plugins(CursorRayPlugin);
        }

     /*   if !app.is_plugin_added::<TransformGizmoPlugin>(){
            
            app.add_plugins(TransformGizmoPlugin);
        }*/

        app
        .add_plugins(clay_tile_block_plugin)        
        .add_plugins(tile_edit_plugin)
        .add_plugins(tile_gizmos_plugin)


        .add_systems(Update,  (

            load_tiles_texture_from_image


            ).chain())


        ; 

        
    }
}



 