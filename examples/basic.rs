use crate::tile_edit::build_tile_layer;
use bevy::prelude::*;
 

 use bevy_clay_tiles::BevyClayTilesPlugin;
 use bevy_clay_tiles::tile_edit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyClayTilesPlugin {})
        //.add_startup_system(setup)
        .add_systems(Startup, build_tile_layer )
        .run();
}

fn setup(mut commands: Commands) {
   
    commands
    .spawn(Camera3dBundle::default());




}
