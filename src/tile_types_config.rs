/*

this is loaded from a RON file


also should incorporate the paths to the height and splat folders for their texture handles...

*/
use crate::ClayTilesConfig;
use bevy::platform::collections::hash_map::HashMap;
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;




#[derive(Resource, Default)]
pub struct ClayTilesConfigResource  (pub ClayTilesConfig) ;

impl ClayTilesConfigResource {

    pub fn get_config(&self) -> &ClayTilesConfig {
        &self.0
    }
}




#[derive(Resource, Default)]
pub struct ClayTilesTypesConfigResource {
    
    
  pub tile_type_data: HashMap<usize, TileTypeConfig>
}

impl ClayTilesTypesConfigResource {

    pub fn new( types_config: &TileTypesConfig ) -> Self {

        let mut tile_type_data = HashMap::with_hasher(Default::default());

        for (i, element) in types_config.tile_types.iter().enumerate() {

            tile_type_data.insert( i , element.clone() );

        }

        Self {
            tile_type_data

        }


    }
}

 


#[derive(  Deserialize, Serialize, Clone)]
pub struct TileTypesConfig {
    
    pub tile_types: Vec<TileTypeConfig>
    
   
}



#[derive(  Deserialize, Serialize, Clone)]
pub struct TileTypeConfig {
    
   pub name: String,
 //  pub diffuse_texture_index: u32,
   pub material_name: String , 
  // pub diffuse_uv_expansion_factor: f32, 
 //  pub diffuse_color_tint: Option<LinearRgba>, 

    
   
}

/*
impl Default for TileTypeConfig {


    fn default() -> Self { 

        Self {
            name: "UnknownTileType".to_string(),
            diffuse_texture_index: 0,
            diffuse_uv_expansion_factor: 1.0,
            diffuse_color_tint: None,
        }

     }
}*/

impl TileTypesConfig {

      pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

}

 
/*
impl TileTypesConfig {


    pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

    
}
*/