 
 
  
use bevy_clay_tiles::clay_tile_block::ClayTileBlockBuilder;
use bevy_clay_tiles::clay_tile_block::ClayTileBlock;
use bevy_clay_tiles::tile_edit::BuildTileTool;
use bevy_clay_tiles::tile_edit::{TileEditingResource,EditingTool as TileEditingTool};
 
//use bevy_clay_tiles::clay_tile_layer::{ClayTileLayer,ClayTileLayerBuildData};
use bevy_clay_tiles::tiles_config::ClayTilesConfig;
use bevy_clay_tiles::tiles::ClayTilesRoot;
  
use bevy::prelude::*;
 

 use bevy_clay_tiles::BevyClayTilesPlugin;
 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyClayTilesPlugin {})
        //.add_startup_system(setup)
        .add_systems(Startup, setup )
        .add_systems(Update, ( rotate_camera ) )
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
    });

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




        /*let tile_operations = vec![
            ClayTileOperation {
                height_layer: 1,
                operation_type: OperationType::Union,  //this doesnt matter
                shape_type: ClayShapeType::Rectangle,
                dimensions: [[-5, -5], [3, 2]],
            },
            ClayTileOperation {
                height_layer: 1,
                operation_type: OperationType::Union,
                shape_type: ClayShapeType::Rectangle,
                dimensions: [[-2, -2], [1, 2]],
            },
        ];*/

            //they need to be in this order ! 

        let polygon_points = vec![
            IVec2::new(0, 0),  
            IVec2::new(2, 0), 
            IVec2::new(2, 2),  
            IVec2::new(0, 2),  
           //  UVec2::new(0, 4),  
            IVec2::new(0, 0), // Closing the loop (same as the first point)
        ];


        let clay_tiles_root =  commands
        .spawn(SpatialBundle::default())
        .insert(ClayTilesConfig::load_from_file("assets/tiles_config.ron").unwrap())
        .insert(ClayTilesRoot::new())
        .id();



        let clay_tile_layer = commands
        .spawn(SpatialBundle::default())
        .insert(ClayTileBlockBuilder {

            polygon_points
        } )
        
        .id();


        commands.entity(clay_tiles_root)
        .add_child(clay_tile_layer);




         /*v
            */

          tile_edit_resource.set_selected_tool(
           Some( TileEditingTool::BuildTile( BuildTileTool::PolygonTileBuild ))
            );

     /* commands.spawn(
        TextBundle::from_section(
            "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
            Press 'P' to toggle perspective for line gizmos\n\
            Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
            Hold 'Up' or 'Down' to change the line width of round gizmos\n\
            Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos\n\
            Press 'A' to show all AABB boxes\n\
            Press 'U' or 'I' to cycle through line styles for straight or round gizmos\n\
            Press 'J' or 'K' to cycle through line joins for straight or round gizmos",
            TextStyle::default(),
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    ); */


}

 

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 20.));
}


