
use crate::clay_tile_block::ClayTileBlockBuilder;
use crate::clay_tile::ClayTileComponent;
use crate::tiles::ClayTilesRoot;
 
use core::f32::consts::PI;
use bevy::{prelude::* };
use geo::{MultiPolygon, BooleanOps, CoordsIter, LineString, OpType, Polygon};
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology::TriangleList;
 

use bevy_mod_raycast::prelude::*;


pub(crate) fn tile_edit_plugin(app: &mut App) {
    app

        .init_resource::<TileEditingResource>()

         .add_event::<BuildGridInteractionEvent>()
        .add_systems(Update, 

            (
                render_tile_build_grid,
                listen_for_input_events,
                handle_polygon_tile_build_events,
                handle_rectangle_tile_build_events, 

                ).chain()

            )
        ;
}




// entity, editToolType, coords, magnitude
#[derive(Event, Debug, Clone)]
pub struct BuildGridInteractionEvent {
     

         coordinates: Vec2, 
         interaction_type: GridInteractionType,

      

}

#[derive(Clone,Debug)]
pub enum GridInteractionType{
    Press,
    Release

}

#[derive(Resource,Default)]
pub struct TileEditingResource{ 
    selected_tool: Option<EditingTool>,
    build_grid_data: TileBuildGridData   // gizmo.. 
}


impl TileEditingResource {

    pub fn get_build_grid_enabled(&self) -> bool{
 
        self.selected_tool.is_some()
    }

    pub fn set_build_layer_height(&mut self, height: u32 ){


        self.build_grid_data.height_offset = height;
    }

    pub fn set_selected_tool(&mut self, tool_type: Option<EditingTool>) {


        self.selected_tool = tool_type;

    }

}


#[derive(Debug, Clone)]
pub enum EditingTool {
    BuildTile( BuildTileTool ),
    ModifyTileHeight(Entity),     
}


#[derive(Debug, Clone)]
pub enum BuildTileTool { 
    RectangleTileBuild , 
    PolygonTileBuild ,   
}


//this is determined by other statefulness
#[derive(Debug, Clone)]
pub enum RectangleTileBuildToolState { 
    PlaceOrigin, // monitors for on click ... spawns an entity 
    PlaceEndpoint(Entity), //monitors for on release 
}
 

//this is determined by other statefulness
#[derive(Debug, Clone)]
pub enum PolygonTileBuildToolState { 
    PlaceOrigin, // monitors for on click ... spawns an entity 
    AddLineSegment(Entity), //monitors for one to be added which is the same as the origin 
}

#[derive(Debug, Clone, Default)]
pub struct TileBuildGridData {

    height_offset: u32,
   // grid_enabled: bool 

}

 
  

//also needs some collision?  use avian w a layer ?? 
  fn render_tile_build_grid( 
    tile_edit_resource: Res<TileEditingResource>,
      mut gizmos: Gizmos,
  ){


     let height_offset  = &tile_edit_resource.build_grid_data.height_offset;
     let grid_enabled = &tile_edit_resource.get_build_grid_enabled();

        if *grid_enabled {

            //bizarre but.. yeah lol . due to quat rot 
          let grid_position = Vec3::new(0.0,0.0, -1.0 *  *height_offset as f32);


           gizmos.grid(
                grid_position,
                Quat::from_rotation_x( PI / 2.),
                UVec2::splat(100),
                Vec2::splat(1.),
                // Light gray
                LinearRgba::gray(0.95),
            );

        }


  }


fn listen_for_input_events (
    tile_edit_resource: Res<TileEditingResource>,

   mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

   cursor_ray: Res<CursorRay>,

   mut build_grid_interaction_evt_writer: EventWriter<BuildGridInteractionEvent>,

 
  
) {


     let build_grid_height  =   tile_edit_resource.build_grid_data.height_offset as f32;
     let grid_enabled =  tile_edit_resource.get_build_grid_enabled();

     if  grid_enabled {
 //   let build_grid_height = 0.0; // this is a flat plane where  X and Z are always 0 

        if let Some(cursor_ray) = **cursor_ray {
            let origin = &cursor_ray.origin; 
            let direction = &cursor_ray.direction;


           // let point_intersecting_build_grid = ;
            if direction.y.abs() > 1e-6 {  // Ensure we're not dividing by zero
                let t = (build_grid_height - origin.y) / direction.y;
                let point_intersecting_build_grid = *origin + *direction * t;
                
                // Check if the left mouse button was just pressed
                if mouse_input.just_pressed(MouseButton::Left) {

                    build_grid_interaction_evt_writer.send(BuildGridInteractionEvent {
                        coordinates: Vec2::new(point_intersecting_build_grid.x, point_intersecting_build_grid.z),
                        interaction_type: GridInteractionType::Press
                    });
                }else if mouse_input.just_released(MouseButton::Left){

                    build_grid_interaction_evt_writer.send(BuildGridInteractionEvent {
                        coordinates: Vec2::new(point_intersecting_build_grid.x, point_intersecting_build_grid.z),
                        interaction_type: GridInteractionType::Release
                    });
                }
            }
        }
     }
}

/*
fn handle_grid_interaction_events(  

    mut evt_reader: EventReader<BuildGridInteractionEvent>
){

    for evt in evt_reader.read() {

 
        info!("got event {:?}", evt );


    }



}
*/


fn handle_polygon_tile_build_events(
    mut commands: Commands,
    mut evt_reader: EventReader<BuildGridInteractionEvent>,
    tile_edit_resource: Res<TileEditingResource>,
    mut builder_query: Query<&mut ClayTileBlockBuilder>,

    root_query: Query<Entity, With< ClayTilesRoot>>,
) {

    let Some(root_entity) = root_query.get_single().ok() else {return};

    if let Some(EditingTool::BuildTile(BuildTileTool::PolygonTileBuild)) = &tile_edit_resource.selected_tool {
        for evt in evt_reader.read() {
            match evt.interaction_type {
                GridInteractionType::Press => {

                    let position = IVec2::new(
                        evt.coordinates.x.round() as i32,
                        evt.coordinates.y.round() as i32,
                    );


                     
                    
                    if let Ok(mut builder) = builder_query.get_single_mut() {
                        // Add the point to the existing builder
                        builder.polygon_points.push(position);
                    } else {
                        // No builder exists, create a new one
                        commands.spawn((
                             SpatialBundle::default(),
                            ClayTileBlockBuilder {
                                polygon_points: vec![position],
                            }
                            // Additional components can be added here
                        )).set_parent( root_entity );
                    }
                }
                _ => {}
            }
        }
    }
}

fn handle_rectangle_tile_build_events(
    mut commands: Commands,
    mut evt_reader: EventReader<BuildGridInteractionEvent>,
    tile_edit_resource: Res<TileEditingResource>,
    mut builder_query: Query<&mut ClayTileBlockBuilder>,
    root_query: Query<Entity, With< ClayTilesRoot>>,
) {

    let Some(root_entity) = root_query.get_single().ok() else {return};


    if let Some(EditingTool::BuildTile(BuildTileTool::RectangleTileBuild)) = &tile_edit_resource.selected_tool {
        for evt in evt_reader.read() {
            match evt.interaction_type {
                GridInteractionType::Press => {
                    let position = IVec2::new(
                        evt.coordinates.x.round() as i32,
                        evt.coordinates.y.round() as i32,
                    );

                    
                    if let Ok(mut builder) = builder_query.get_single_mut() {
                        // Replace the existing point with the new start point
                        builder.polygon_points.clear();
                        builder.polygon_points.push(position);
                    } else {
                        // No builder exists, create a new one with the first point
                        commands.spawn((
                            SpatialBundle::default(),
                            ClayTileBlockBuilder {
                                polygon_points: vec![position],
                            },
                            // Additional components can be added here
                        )).set_parent( root_entity );
                    }
                }
                GridInteractionType::Release => {
                    if let Ok(mut builder) = builder_query.get_single_mut() {
                        if let Some(&start_point) = builder.polygon_points.first() {
                            
                            let end_point = IVec2::new(
                                evt.coordinates.x.round() as i32,
                                evt.coordinates.y.round() as i32,
                            );

                            // Calculate the other two corners of the rectangle
                            let top_right = IVec2::new(end_point.x, start_point.y);
                            let bottom_left = IVec2::new(start_point.x, end_point.y);

                            // Complete the rectangle by adding the other points
                            builder.polygon_points.push(top_right);
                            builder.polygon_points.push(end_point);
                            builder.polygon_points.push(bottom_left);
                            builder.polygon_points.push(start_point); // Close the rectangle

                            info!("{} {}", start_point, end_point);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
