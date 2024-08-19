 
 
  
use bevy_clay_tiles::tile_edit::ModifyTileTool;
use transform_gizmo_bevy::GizmoCamera;
use bevy_clay_tiles::clay_tile_block::ClayTileBlockBuilder;
use bevy_clay_tiles::clay_tile_block::ClayTileBlock;
use bevy_clay_tiles::tile_edit::BuildTileTool;
use bevy_clay_tiles::tile_edit::{TileEditingResource,EditingTool as TileEditingTool};
 
//use bevy_clay_tiles::clay_tile_layer::{ClayTileLayer,ClayTileLayerBuildData};
use bevy_clay_tiles::tiles_config::ClayTilesConfig;
 
  
use bevy::prelude::*;
 

 use bevy_clay_tiles::BevyClayTilesPlugin;
 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyClayTilesPlugin {
            config: ClayTilesConfig::load_from_file("assets/tiles_config.ron").unwrap()
        })
        //.add_startup_system(setup)
        .add_systems(Startup, setup )
        .add_systems(Update, rotate_camera  )
        .run();
}

fn setup( 
    mut commands: Commands,

    // mut config_store: ResMut<GizmoConfigStore>,


     mut tile_edit_resource: ResMut<TileEditingResource>,
     ) {
   
     commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 7.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(GizmoCamera);

      // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

        commands.insert_resource(AmbientLight {
        color: Color::srgb(1.0,1.0,1.0),
        brightness: 700.0,
    });
 

 
      //typically you wont define meshes by points manually , this is just an example ..

        let polygon_points = vec![
            IVec2::new(0, 0),  
            IVec2::new(2, 0), 
            IVec2::new(2, 2),  
            IVec2::new(0, 2),  
           //  UVec2::new(0, 4),  
            IVec2::new(0, 0), // Closing the loop (same as the first point)
        ];
 

         commands
        .spawn(SpatialBundle::default())
        .insert(ClayTileBlockBuilder {

            polygon_points,

            ..default()
        } )
         ;


        

 
        /*
        In your editor, your tooling/controls will modify the tile_edit_resource. 
        this will allow you to edit tiles in real time

        When you are ready to save/load your tiles, just write the 'ClayTileBlock' component with serde to hard-disk and deserialize it back to load.
        
        */
          tile_edit_resource.set_selected_tool(
           Some( TileEditingTool::BuildTile( BuildTileTool::RectangleTileBuild ))
        //  Some( TileEditingTool::ModifyTile ( ModifyTileTool::ModifyTileHeight ))
            );

      

}

 

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 20.));
}


