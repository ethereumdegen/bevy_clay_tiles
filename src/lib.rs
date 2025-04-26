 
 
use crate::tile_types_config::ClayTilesTypesConfigResource;
use crate::tile_types_config::ClayTilesConfigResource;
use crate::tile_types_config::TileTypeConfig;
use bevy::platform::collections::hash_map::HashMap;
//use bevy_material_tool::BevyMaterialToolPlugin;
use crate::tile_types_config::TileTypesConfig;
//use crate::tiles_texturing::ClayTilesTypesConfigResource;
use crate::modify_tiles::modify_tiles_plugin;
//use transform_gizmo_bevy::TransformGizmoPlugin; 

use bevy::asset::{AssetPath, LoadState};
use crate::tiles_config::ClayTilesConfig;
//use crate::tiles_texturing::ClayTilesTexturingResource;
// use bevy_mod_raycast::prelude::CursorRayPlugin;
use crate::tile_edit::tile_edit_plugin;

use crate::clay_tile_block::clay_tile_block_plugin;
 
use crate::tile_material::TileMaterialExtension;
use crate::tile_material::TILE_SHADER_HANDLE;
//use crate::tile_edit::tile_edit_plugin; 
use bevy::{asset::load_internal_asset, prelude::*};
 

 
 //pub mod tiles_texturing;
 
 pub mod clay_tile_block; 
 pub mod tile_material;
 pub mod tiles_config;
 pub mod tile_types_config;
 pub mod tile_edit;
 pub mod pre_mesh;
 pub mod clay_tile;
 pub mod modify_tiles;
 

pub struct BevyClayTilesPlugin {
    pub config: ClayTilesConfig
}
 
impl Plugin for BevyClayTilesPlugin {
    fn build(&self, app: &mut App) {

        //app.init_resource::<ClayTilesTexturingResource>();
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

        
           /* 
       if !app.is_plugin_added::<CursorRayPlugin>() {
             app.add_plugins(CursorRayPlugin);
        }

         if !app.is_plugin_added::<TransformGizmoPlugin>(){
            
            app.add_plugins(TransformGizmoPlugin);
        }*/

        app
        .add_plugins(clay_tile_block_plugin)        
        .add_plugins(tile_edit_plugin)
        .add_plugins(modify_tiles_plugin)
     //   .add_plugins(tiles_texturing::tiles_texturing_plugin)


        


        ; 

        
    }
}



