/*

this is loaded from a RON file


also should incorporate the paths to the height and splat folders for their texture handles...

*/
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Component, Deserialize, Serialize, Clone)]
pub struct ClayTilesConfig {
    
   
  //  pub texture_image_sections: u32,
 //   pub diffuse_texture_path: PathBuf,
 //   pub normal_texture_path: PathBuf,
    pub tile_types_config_path: PathBuf,
}

impl Default for ClayTilesConfig {
    fn default() -> Self {
        Self {
           
            
         //   texture_image_sections: 4, 
          //  diffuse_texture_path: "diffuse/".into(),
         //   normal_texture_path: "normal/".into(),
            tile_types_config_path: "tile_types.ron".into()
            
         //   collider_data_folder_path: "collider/".into(),
        }
    }
}



//this may break in a production build ? 

impl ClayTilesConfig {
    pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

    pub fn get_tile_types_config_path( &self ) -> &PathBuf {
        &self.tile_types_config_path 


    } 

    
}
