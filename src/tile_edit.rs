
use crate::clay_tile_block::ClayTileBlockBuilder;
use crate::clay_tile::ClayTileComponent;
 
 
use core::f32::consts::PI;
use bevy::color::palettes::tailwind;
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
                update_build_grid_horizontal_offset,
                render_tile_build_grid,
                render_cursor_gizmo,
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
    Release,
     Cancel

}

#[derive(Resource,Default)]
pub struct TileEditingResource{ 
    selected_tool: Option<EditingTool>,
    build_grid_data: TileBuildGridData  ,  

    selected_tile_type: u32 ,
    new_tile_parent_entity: Option<Entity>
}


impl TileEditingResource {

    pub fn get_build_grid_enabled(&self) -> bool{
 
        self.selected_tool.is_some()
    }

    pub fn set_build_grid_horizontal_offset( &mut self, offset: Vec2 ){

        self.build_grid_data.horizontal_offset = offset; 
    }

    pub fn set_build_tile_type(&mut self, tile_type: u32){


        self.selected_tile_type = tile_type;
    }


    pub fn get_build_tile_type(& self) -> u32{
 

        self.selected_tile_type  
    }

    pub fn set_build_layer_height(&mut self, height: u32 ){


        self.build_grid_data.height_offset = height;
    }

    pub fn get_build_layer_height(& self ) -> u32{


        self.build_grid_data.height_offset   
    }

    pub fn set_selected_tool(&mut self, tool_type: Option<EditingTool>) {


        self.selected_tool = tool_type;

    }

    pub fn set_new_tile_parent_entity(&mut self, parent: Option<Entity>) {


        self.new_tile_parent_entity = parent;

    }


    pub fn get_new_tile_parent_entity(& self ) -> Option<Entity>{


        self.new_tile_parent_entity  

    }

}


#[derive(Debug, Clone)]
pub enum EditingTool {
    BuildTile( BuildTileTool ),
    ModifyTile (ModifyTileTool)    
}


#[derive(Debug, Clone)]
pub enum BuildTileTool { 
    RectangleTileBuild , 
    PolygonTileBuild ,   
}



#[derive(Debug, Clone)]
pub enum ModifyTileTool { 
    ModifyTileHeight , 
    ModifyTileBevel ,   
    ModifyTileType, 
}



//this is determined by other statefulness
/*
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
*/

#[derive(Debug, Clone, Default)]
pub struct TileBuildGridData {

    height_offset: u32,

    horizontal_offset: Vec2, 
   // grid_enabled: bool 

}

 
  

//also needs some collision?  use avian w a layer ?? 
  fn render_tile_build_grid( 
    tile_edit_resource: Res<TileEditingResource>,
      mut gizmos: Gizmos,
  ){

     let position_offset  = &tile_edit_resource.build_grid_data.horizontal_offset.xy();
     let height_offset  = &tile_edit_resource.build_grid_data.height_offset;
     let grid_enabled = &tile_edit_resource.get_build_grid_enabled();

     let x_offset = position_offset.x;
     let z_offset = position_offset.y;

        if *grid_enabled {

            //bizarre but.. yeah lol . due to quat rot 
          let grid_position = Vec3::new(x_offset,z_offset, -1.0 *  *height_offset as f32);


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


                 if mouse_input.just_pressed(MouseButton::Right) {

                    build_grid_interaction_evt_writer.send(BuildGridInteractionEvent {
                        coordinates: Vec2::new(point_intersecting_build_grid.x, point_intersecting_build_grid.z),
                        interaction_type: GridInteractionType::Cancel
                    });
                }


            }
        }
     }
}




fn render_cursor_gizmo (
    tile_edit_resource: Res<TileEditingResource>,
 

   cursor_ray: Res<CursorRay>,

   mut gizmos: Gizmos,
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
                    


                    let rounded_position = IVec2::new(
                        point_intersecting_build_grid.x.round() as i32,
                        point_intersecting_build_grid.z.round() as i32,
                    ); 

                    let position = Vec3::new(rounded_position.x as f32, 1.0 * build_grid_height, rounded_position.y as f32);

                   
                    let radius = 0.1;
                    let rotation = Quat::IDENTITY;
                    let color = tailwind::AMBER_400  ;

                    gizmos.sphere(position, rotation, radius, color) ;

                  
                
            }
        }
     }
}


fn update_build_grid_horizontal_offset (
    mut tile_edit_resource: ResMut<TileEditingResource>,
 

   cursor_ray: Res<CursorRay>,

//   mut gizmos: Gizmos,
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
                    


                    let rounded_position = IVec2::new(
                        point_intersecting_build_grid.x.round() as i32,
                        point_intersecting_build_grid.z.round() as i32,
                    ); 

                   // let position = Vec3::new(rounded_position.x as f32, 0.0, rounded_position.y as f32);
                    tile_edit_resource.set_build_grid_horizontal_offset( Vec2::new(rounded_position.x as f32,  rounded_position.y as f32 )) ;
                
                  
                
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

   // root_query: Query<Entity, With< ClayTilesRoot>>,
) {

    let new_tile_parent_entity = tile_edit_resource.get_new_tile_parent_entity();

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

                        let height_level = tile_edit_resource.get_build_layer_height() ;
                        let tile_type_index = tile_edit_resource.get_build_tile_type();

                        // No builder exists, create a new one
                        let block_builder_entity = commands.spawn((
                             SpatialBundle::default(),
                              ClayTileBlockBuilder {
                                polygon_points: vec![position],

                                height_level,
                                tile_type_index,

                            }
                            // Additional components can be added here
                        )).id();

                        if let Some( new_tile_parent_entity ) = new_tile_parent_entity {

                            commands.entity(block_builder_entity).set_parent( new_tile_parent_entity );
                        }
                    }
                }

                GridInteractionType::Cancel => {


                    let position = IVec2::new(
                        evt.coordinates.x.round() as i32,
                        evt.coordinates.y.round() as i32,
                    ); 
                    
                    if let Ok(mut builder) = builder_query.get_single_mut() {
                        // Add the point to the existing builder
                        builder.polygon_points.clear();
                        builder.polygon_points.push(position);
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
  // root_query: Query<Entity, With< ClayTilesRoot>>,
) {

  let new_tile_parent_entity = tile_edit_resource.get_new_tile_parent_entity();



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

                        let height_level = tile_edit_resource.get_build_layer_height() ;
                        let tile_type_index = tile_edit_resource.get_build_tile_type();

                        // No builder exists, create a new one with the first point
                        let block_builder_entity = commands.spawn((
                            SpatialBundle::default(),
                            ClayTileBlockBuilder {
                                polygon_points: vec![position],
                                height_level,
                                tile_type_index
                            },
                            // Additional components can be added here
                        )).id() ;

                         if let Some( new_tile_parent_entity ) = new_tile_parent_entity {

                            commands.entity(block_builder_entity).set_parent( new_tile_parent_entity );
                        }
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
