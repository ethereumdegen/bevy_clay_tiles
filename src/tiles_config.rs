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
    
   // pub height_scale: f32,

    

    pub texture_uv_expansion_factor: f32, 

    pub texture_image_sections: u32,
    pub diffuse_folder_path: PathBuf,
  //  pub normal_folder_path: PathBuf,
   
  //  pub collider_data_folder_path: PathBuf,
}

impl Default for ClayTilesConfig {
    fn default() -> Self {
        Self {
            // chunk_width: 64.0 ,
            

            texture_uv_expansion_factor : 16.0,

            
            texture_image_sections: 4, 
            diffuse_folder_path: "diffuse/".into(),
            
         //   collider_data_folder_path: "collider/".into(),
        }
    }
}

impl ClayTilesConfig {
    pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

    
}
