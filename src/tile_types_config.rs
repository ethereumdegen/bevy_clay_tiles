/*

this is loaded from a RON file


also should incorporate the paths to the height and splat folders for their texture handles...

*/
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(  Deserialize, Serialize, Clone)]
pub struct TileTypesConfig {
    
    tile_types: Vec<TileTypeConfig>
    
   
}



#[derive(  Deserialize, Serialize, Clone)]
pub struct TileTypeConfig {
    
   name: String,
   diffuse_texture_index: usize,

   diffuse_uv_expansion_factor: f32, 
   diffuse_color_tint: Option<Color>, 

    
   
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