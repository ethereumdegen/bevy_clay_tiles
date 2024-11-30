 
 
   
use bevy_clay_tiles::clay_tile_block::ClayTileMaterial;
use bevy_clay_tiles::tile_edit::ModifyTileTool;
use bevy_material_wizard::BevyMaterialWizardPlugin;
 // use transform_gizmo_bevy::GizmoCamera;
use bevy_clay_tiles::clay_tile_block::ClayTileBlockBuilder;
use bevy_clay_tiles::clay_tile_block::ClayTileBlock;
use bevy_clay_tiles::tile_edit::BuildTileTool;
use bevy_clay_tiles::tile_edit::{TileEditingResource,EditingTool as TileEditingTool};
 
//use bevy_clay_tiles::clay_tile_layer::{ClayTileLayer,ClayTileLayerBuildData};
use bevy_clay_tiles::tiles_config::ClayTilesConfig;
 
  
use bevy::prelude::*;
 

 use bevy_clay_tiles::BevyClayTilesPlugin;
 
 use bevy_material_wizard::material_overrides::MaterialOverrideComponent;
 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyMaterialWizardPlugin{
            material_defs_folder_path: "assets/material_overrides".to_string(), 
        })
        .add_plugins(BevyClayTilesPlugin {
            config: ClayTilesConfig::load_from_file("assets/tiles_config.ron").unwrap()
        })
        //.add_startup_system(setup)
        .add_systems(Startup, setup )
        

        .add_systems( Startup, load_sample_block)
        .add_systems(Update, rotate_camera  )

         .add_systems(Update, update_directional_light_position)
         .add_systems(Update, add_material_handles)
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
    }) ; 

         

        commands.insert_resource(AmbientLight {
        color: Color::srgb(1.0,1.0,1.0),
        brightness: 122.0,
    });

        // light
    commands.spawn( 
         ( 
            DirectionalLight {
            //shadow_depth_bias: 0.5,
            //shadow_normal_bias: 0.5,
            illuminance: 700.0,  
            color: Color::srgba(1.0, 1.0, 1.0, 1.0),

            ..default()
             },
             Transform::from_xyz(4.0, 6.0, 4.0),
        )
      );
    // light
 

 
      //typically you wont define meshes by points manually , this is just an example ..

      

 
        /*
        In your editor, your tooling/controls will modify the tile_edit_resource. 
        this will allow you to edit tiles in real time

        When you are ready to save/load your tiles, just write the 'ClayTileBlock' component with serde to hard-disk and deserialize it back to load.
        
        */
          tile_edit_resource.set_selected_tool(
           Some( TileEditingTool::BuildTile( BuildTileTool::RectangleTileBuild ))
           // Some( TileEditingTool::ModifyTile ( ModifyTileTool::ModifyDragSides ))
            );

      

}

 

fn load_sample_block(
    mut commands:Commands,
    ){

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


        


}

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_secs() / 20.));
}




fn update_directional_light_position(
    mut query: Query<&mut Transform, With<DirectionalLight>>,
   
    time: Res<Time>,
) {

    let current_time = time.elapsed();


 //   let delta_time = time.delta_seconds();
    
    let SECONDS_IN_A_CYCLE = 10.0;

    let angle = (current_time.as_millis() as f32 / (SECONDS_IN_A_CYCLE* 1000.0) ) * std::f32::consts::PI * 2.0; // Convert time to radians
   
    let radius = 20.0; // Adjust the radius of the sun's orbit
    let x = angle.cos() * radius;
    let y = angle.sin() * radius + 10.0; // Adjust the height of the sun
    let z = 0.0;

    for mut transform in query.iter_mut() {

        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}


fn add_material_handles(
    mut commands:Commands,

    block_query: Query<(Entity, &ClayTileMaterial), Added<ClayTileMaterial>>
){

    for (tile_entity, tile_material_comp) in block_query.iter(){

        let material_name = &tile_material_comp.material_name; 


        commands.get_entity(tile_entity).map( |mut cmd| { cmd.insert( MaterialOverrideComponent {
            material_override:  material_name.clone()
        }  ); } );

    }

}