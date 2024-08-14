 
use bevy::time::common_conditions::on_timer;
use bevy::{asset::load_internal_asset, prelude::*};
 

use std::time::Duration;
 
 pub mod tile_edit;
 
 

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
        
 
    

        
    }
}
