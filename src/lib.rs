 
use std::f32::consts::PI;
use crate::clay_tile_layer::clay_tile_layer_plugin;
use crate::tiles::load_tiles_texture_from_image;
use crate::tile_material::TileMaterialExtension;
use crate::tile_material::TILE_SHADER_HANDLE;
//use crate::tile_edit::tile_edit_plugin; 
use bevy::{asset::load_internal_asset, prelude::*};
 

//use std::time::Duration;
 
 pub mod tiles;
 pub mod clay_tile_layer;
 pub mod tile_material;
 pub mod tiles_config;
 //pub mod tile_edit;
 pub mod pre_mesh;
 pub mod clay_tile;
 pub mod clay_tile_operation;
 
 

pub struct BevyClayTilesPlugin {
    
}

impl Default for BevyClayTilesPlugin {
    fn default() -> Self {
        Self {
          //  task_update_rate: Duration::from_millis(250),
        }
    }
}
impl Plugin for BevyClayTilesPlugin {
    fn build(&self, app: &mut App) {


        load_internal_asset!(
            app,
            TILE_SHADER_HANDLE,
            "shaders/tile.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<TileMaterialExtension>::default());
        


        app
        .add_plugins(clay_tile_layer_plugin)


        .add_systems(Update,  (

            load_tiles_texture_from_image


            ).chain())


        ;
        
 
    

        
    }
}





pub fn draw_grid_gizmo(
  mut gizmos: Gizmos,

   ){ 


       gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(100),
        Vec2::splat(1.),
        // Light gray
        LinearRgba::gray(0.95),
    );


     // UVec2::new(10, 10),
      //  Vec2::splat(2.),


}
