
use crate::clay_tile_block::ClayTileBlock;
use crate::clay_tile_block::ClayTileBlockBuilder;
use crate::clay_tile::ClayTileComponent;
 
 
use core::f32::consts::PI;
use bevy::color::palettes::tailwind;
use bevy::math::VectorSpace;
use bevy::{prelude::* };
use geo::{MultiPolygon, BooleanOps, CoordsIter, LineString, OpType, Polygon};
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology::TriangleList;
 

use bevy_mod_raycast::prelude::*;


pub(crate) fn tile_edit_plugin(app: &mut App) {
    app

        .init_resource::<TileEditingResource>()
        .init_resource::<TileBuildPreviewResource>()

        .add_event::<TileSelectionEvent>()

        .add_event::<BuildGridInteractionEvent>()
        .add_systems(Update, 

            (
           

                update_build_grid_horizontal_offset,
                render_tile_build_grid,
                render_cursor_gizmo,
                listen_for_input_events,
                handle_polygon_tile_build_events,
                handle_rectangle_tile_build_events, 
                handle_linear_tile_build_events,

                update_tile_build_preview_rectangle,

                   update_tile_build_preview_linear,



              

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


#[derive(Event, Debug, Clone)]
pub enum TileSelectionEvent {
      
    SelectTile(Entity),
    DeselectTiles, 
 
}


#[derive(Clone,Debug)]
pub enum GridInteractionType{
    Press,
    Release,
     Cancel

}


#[derive(Resource ,Default)]
pub struct TileBuildPreviewResource{ 
     
    drag_origin: Option<IVec2>,
    current_position: Option<IVec2> ,

}



#[derive(Resource )]
pub struct TileEditingResource{ 
    selected_tool: Option<EditingTool>,
    tool_enabled: bool,

    build_grid_data: TileBuildGridData  ,  

    selected_tile_type: usize ,
    build_mesh_height: f32, 

    new_tile_parent_entity: Option<Entity>,

    pub selected_tile: Option<Entity>,
}

impl Default for TileEditingResource { 

    fn default() -> Self { 
     Self {
        tool_enabled : true ,
        selected_tool: None,
        build_grid_data: TileBuildGridData::default(),
        selected_tile_type: 0 ,
        build_mesh_height : 0.2 ,
        new_tile_parent_entity: None ,
        selected_tile: None 

        }
    }


}


impl TileEditingResource {


    pub fn set_tool_enabled(&mut self, enabled : bool ) {
 
        self.tool_enabled = enabled ;
    }


    pub fn get_build_grid_enabled(&self) -> bool{
 
        self.selected_tool.is_some() && self.tool_enabled
    }

    pub fn set_build_grid_horizontal_offset( &mut self, offset: Vec2 ){

        self.build_grid_data.horizontal_offset = offset; 
    }

    pub fn set_build_tile_type(&mut self, tile_type: usize){


        self.selected_tile_type = tile_type;
    }


    pub fn get_build_tile_type(& self) -> usize{
 

        self.selected_tile_type  
    }


    pub fn get_build_mesh_height(&self) -> f32 {

        self.build_mesh_height 
    }

       pub fn set_build_mesh_height(&mut self, height: f32)   {

        self.build_mesh_height = height;
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

    pub fn able_to_select_tiles(&self) -> bool {


        match self.selected_tool{

            Some(EditingTool::ModifyTile(..)) => true,
            _ => false 
        }



    }

    pub fn show_cursor_gizmo(&self) -> bool {

     

         if self.get_build_grid_enabled() == false {
            return false ;
        }



        match self.selected_tool{

                 Some(EditingTool::BuildTile(..) ) => true,
            _ => false 
        }
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
    LinearTileBuild, 
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
     let grid_enabled = tile_edit_resource.get_build_grid_enabled();

     let x_offset = position_offset.x;
     let z_offset = position_offset.y;

        if grid_enabled {

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

    let tool_enabled = tile_edit_resource.tool_enabled;
 

     let build_grid_height  =   tile_edit_resource.build_grid_data.height_offset as f32;
     let grid_enabled =  tile_edit_resource.get_build_grid_enabled();

     if  grid_enabled  && tool_enabled {
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

     let render_cursor_gizmo= tile_edit_resource.show_cursor_gizmo();


     if  grid_enabled && render_cursor_gizmo {
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
    mut builder_query: Query<(Entity, &mut ClayTileBlockBuilder)>,

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
                    
                    if let Ok((builder_entity, mut builder)) = builder_query.get_single_mut() {
                        // Add the point to the existing builder
                        builder.polygon_points.push(position);
                    } else {

                        let height_level = tile_edit_resource.get_build_layer_height() ;
                        let tile_type_index = tile_edit_resource.get_build_tile_type();
                        let mesh_height = tile_edit_resource.get_build_mesh_height();

                        // No builder exists, create a new one
                        let block_builder_entity = commands.spawn((
                             SpatialBundle::default(),
                              ClayTileBlockBuilder {
                                polygon_points: vec![position],

                                height_level,
                                tile_type_index,
                                mesh_height,

                            }
                            // Additional components can be added here
                        )).id();

                       // if let Some( new_tile_parent_entity ) = new_tile_parent_entity {

                        //    commands.entity(block_builder_entity).set_parent( new_tile_parent_entity );
                        //}
                    }
                }

                GridInteractionType::Cancel => {
 
                    
                    if let Ok((builder_entity, mut builder)) = builder_query.get_single_mut() {
                      
                       if let Some(mut cmd) = commands.get_entity(builder_entity){
                           cmd.despawn_recursive();
                       }
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
    mut builder_query: Query<(Entity,&mut ClayTileBlockBuilder)>,
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

                    
                    if let Ok((builder_entity, mut builder)) = builder_query.get_single_mut() {
                        // Replace the existing point with the new start point
                        builder.polygon_points.clear();
                        builder.polygon_points.push(position);
                    } else {

                        let height_level = tile_edit_resource.get_build_layer_height() ;
                        let tile_type_index = tile_edit_resource.get_build_tile_type();
                          let mesh_height = tile_edit_resource.get_build_mesh_height();

                        // No builder exists, create a new one with the first point
                        let block_builder_entity = commands.spawn((
                            SpatialBundle::default(),
                            ClayTileBlockBuilder {
                                polygon_points: vec![position],
                                height_level,
                                tile_type_index,
                                mesh_height 
                            },
                            // Additional components can be added here
                        )).id() ;

                         if let Some( new_tile_parent_entity ) = new_tile_parent_entity {

                            commands.entity(block_builder_entity).set_parent( new_tile_parent_entity );
                        }
                    }
                }
                GridInteractionType::Release => {
                    if let Ok((builder_entity, mut builder)) = builder_query.get_single_mut() {
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

                 GridInteractionType::Cancel => {
 
                    
                    if let Ok((builder_entity, mut builder)) = builder_query.get_single_mut() {
                      
                       if let Some(mut cmd) = commands.get_entity(builder_entity){
                           cmd.despawn_recursive();
                       }
                    }  



                }

                _ => {}
            }
        }
    }
}



fn handle_linear_tile_build_events(
    mut commands: Commands,
    mut evt_reader: EventReader<BuildGridInteractionEvent>,
    tile_edit_resource: Res<TileEditingResource>,
    mut builder_query: Query<(Entity,&mut ClayTileBlockBuilder)>,
  // root_query: Query<Entity, With< ClayTilesRoot>>,
) {

  let new_tile_parent_entity = tile_edit_resource.get_new_tile_parent_entity();



    if let Some(EditingTool::BuildTile(BuildTileTool::LinearTileBuild)) = &tile_edit_resource.selected_tool {
        for evt in evt_reader.read() {
            match evt.interaction_type {
                GridInteractionType::Press => {
                    let position = IVec2::new(
                        evt.coordinates.x.round() as i32,
                        evt.coordinates.y.round() as i32,
                    );

                    
                    if let Ok((builder_entity, mut builder)) = builder_query.get_single_mut() {
                        // Replace the existing point with the new start point
                        builder.polygon_points.clear();
                        builder.polygon_points.push(position);
                    } else {

                        let height_level = tile_edit_resource.get_build_layer_height() ;
                        let tile_type_index = tile_edit_resource.get_build_tile_type();
                          let mesh_height = tile_edit_resource.get_build_mesh_height();

                        // No builder exists, create a new one with the first point
                        let block_builder_entity = commands.spawn((
                            SpatialBundle::default(),
                            ClayTileBlockBuilder {
                                polygon_points: vec![position],
                                height_level,
                                tile_type_index,
                                mesh_height 
                            },
                            // Additional components can be added here
                        )).id() ;

                         if let Some( new_tile_parent_entity ) = new_tile_parent_entity {

                            commands.entity(block_builder_entity).set_parent( new_tile_parent_entity );
                        }
                    }
                }
                GridInteractionType::Release => {
                    if let Ok((builder_entity, mut builder)) = builder_query.get_single_mut() {
                        if let Some(&start_point) = builder.polygon_points.first() {
                            
                            let end_point = IVec2::new(
                                evt.coordinates.x.round() as i32,
                                evt.coordinates.y.round() as i32,
                            );

                            
                           let direction = end_point - start_point;
                            let is_diagonal = direction.x.abs() == direction.y.abs();

                            let mut thickness = 1; // Default thickness

                            // Scale thickness based on angle to ensure minimum wall thickness
                            let angle = (direction.y as f32).atan2(direction.x as f32).abs();
                            let min_thickness = 0.5; // Minimum thickness is half the tile width

                            // Increase thickness for shallow angles
                            if !is_diagonal {
                                if direction.x != 0 && direction.y != 0 {
                                    thickness = (min_thickness / angle.cos()).max(1.0).round() as i32;
                                }
                            }

                            let mut points = vec![start_point];

                            if is_diagonal {
                                let offset = IVec2::new(-direction.y.signum(), direction.x.signum()) * thickness;

                                points.push(start_point + offset);
                                points.push(end_point + offset);
                                points.push(end_point);
                            } else {
                                if direction.x != 0 {
                                    points.push(IVec2::new(start_point.x, start_point.y + thickness));
                                    points.push(IVec2::new(end_point.x, end_point.y + thickness));
                                } else if direction.y != 0 {
                                    points.push(IVec2::new(start_point.x + thickness, start_point.y));
                                    points.push(IVec2::new(end_point.x + thickness, end_point.y));
                                }
                                points.push(end_point);
                            }

                            points.push(start_point);

                            builder.polygon_points = points;




                        }
                    }
                }

                 GridInteractionType::Cancel => {
 
                    
                    if let Ok((builder_entity, mut builder)) = builder_query.get_single_mut() {
                      
                       if let Some(mut cmd) = commands.get_entity(builder_entity){
                           cmd.despawn_recursive();
                       }
                    }  



                }

                _ => {}
            }
        }
    }
}


fn update_tile_build_preview_rectangle(
 

    tile_edit_resource: Res<TileEditingResource>,


  //    tile_build_preview: Res<TileBuildPreviewResource>,
      builder_query: Query<&  ClayTileBlockBuilder>,


   cursor_ray: Res<CursorRay>,

   mut gizmos: Gizmos,

   time: Res<Time>
  
) {

  let new_tile_parent_entity = tile_edit_resource.get_new_tile_parent_entity();


  let time_elapsed = time.elapsed_seconds ();
  let color_factor = (time_elapsed % 6.0)/ 6.0;


  let current_color = LinearRgba::rgb(
    ((color_factor  + 0.0)  )% 1.0, 
    ((color_factor  + 0.33)  ) % 1.0, 
    ((color_factor  + 0.66)   )% 1.0
    );

  let Some(EditingTool::BuildTile(BuildTileTool::RectangleTileBuild)) = &tile_edit_resource.selected_tool else{return};
     

     let build_grid_height  =   tile_edit_resource.build_grid_data.height_offset as f32;
     let grid_enabled =  tile_edit_resource.get_build_grid_enabled();

     let render_cursor_gizmo= tile_edit_resource.show_cursor_gizmo();

     let mut intersection_point:Option<IVec2> = None ;

     if  grid_enabled && render_cursor_gizmo {
 //   let build_grid_height = 0.0; // this is a flat plane where  X and Z are always 0 

        if let Some(cursor_ray) = **cursor_ray {
            let origin = &cursor_ray.origin; 
            let direction = &cursor_ray.direction;


           // let point_intersecting_build_grid = ;
            if direction.y.abs() > 1e-6 {  // Ensure we're not dividing by zero
                let t = (build_grid_height - origin.y) / direction.y;
                let point_intersecting_build_grid = *origin + *direction * t;
                    


                    intersection_point = Some( IVec2::new(
                        point_intersecting_build_grid.x.round() as i32,
                        point_intersecting_build_grid.z.round() as i32,
                    ) ); 

                   // intersection_position = Some( IVec2::new(rounded_position.x ,     rounded_position.y  ) );
 
                  
                
            }
        }
     }  

     let mut origin_point = None;

     if let Some( clay_tile_block_builder ) = builder_query.get_single().ok() {
 
        origin_point = clay_tile_block_builder.get_origin_point();

     }




     if let Some(origin_point) = origin_point {


        if let Some(intersection_point) = intersection_point {
  
            let dimensions = intersection_point.clone() - *origin_point ;
            let origin_f32 = Vec3::new( origin_point.x as f32,  1.0 * build_grid_height + 0.01 , origin_point.y as f32);
            let endpoint_f32  = Vec3::new( intersection_point.x as f32,  1.0 * build_grid_height + 0.01 , intersection_point.y as f32);

            let centroid = origin_f32.lerp( endpoint_f32 , 0.5);

             gizmos.rect(
                centroid,
                Quat::from_rotation_x(PI / 2.),
                Vec2::new( dimensions.x as f32 , dimensions.y as f32)   ,
                current_color,
            );

 



        } 


     }



 
                   
  
              
    
}





fn update_tile_build_preview_linear(
 

    tile_edit_resource: Res<TileEditingResource>,
 
    builder_query: Query<&  ClayTileBlockBuilder>,


   cursor_ray: Res<CursorRay>,

   mut gizmos: Gizmos,

   time: Res<Time>
  
) {

  let new_tile_parent_entity = tile_edit_resource.get_new_tile_parent_entity();


  let time_elapsed = time.elapsed_seconds ();
  let color_factor = (time_elapsed % 6.0)/ 6.0;


  let current_color = LinearRgba::rgb(
    ((color_factor  + 0.0)  )% 1.0, 
    ((color_factor  + 0.33)  ) % 1.0, 
    ((color_factor  + 0.66)   )% 1.0
    );

  let Some(EditingTool::BuildTile(BuildTileTool::LinearTileBuild)) = &tile_edit_resource.selected_tool else{return};
     

     let build_grid_height  =   tile_edit_resource.build_grid_data.height_offset as f32;
     let grid_enabled =  tile_edit_resource.get_build_grid_enabled();

     let render_cursor_gizmo= tile_edit_resource.show_cursor_gizmo();

     let mut intersection_point:Option<IVec2> = None ;

     if  grid_enabled && render_cursor_gizmo {
 //   let build_grid_height = 0.0; // this is a flat plane where  X and Z are always 0 

        if let Some(cursor_ray) = **cursor_ray {
            let origin = &cursor_ray.origin; 
            let direction = &cursor_ray.direction;


           // let point_intersecting_build_grid = ;
            if direction.y.abs() > 1e-6 {  // Ensure we're not dividing by zero
                let t = (build_grid_height - origin.y) / direction.y;
                let point_intersecting_build_grid = *origin + *direction * t;
                    


                    intersection_point = Some( IVec2::new(
                        point_intersecting_build_grid.x.round() as i32,
                        point_intersecting_build_grid.z.round() as i32,
                    ) ); 

                   // intersection_position = Some( IVec2::new(rounded_position.x ,     rounded_position.y  ) );
 
                  
                
            }
        }
     }  

     let mut origin_point = None;

     if let Some( clay_tile_block_builder ) = builder_query.get_single().ok() {
 
        origin_point = clay_tile_block_builder.get_origin_point();

     }




     if let Some(origin_point) = origin_point {


        if let Some(intersection_point) = intersection_point {
  
            let direction = (intersection_point - *origin_point).as_vec2();
            let length = direction.length();
            let thickness = 0.5; // Set the thickness of the line (can be adjusted)

            let origin_f32 = Vec3::new(
                origin_point.x as f32,
                build_grid_height + 0.01,
                origin_point.y as f32,
            );
            let endpoint_f32 = Vec3::new(
                intersection_point.x as f32,
                build_grid_height + 0.01,
                intersection_point.y as f32,
            );

            let midpoint = origin_f32.lerp(endpoint_f32, 0.5);
           let rotation_angle = direction.y.atan2(direction.x)  ;


           
           let total_rotation =  Quat::from_rotation_x(PI / 2.) *   Quat::from_rotation_z(rotation_angle);

            gizmos.rect(
                midpoint,
                total_rotation, //Quat::from_rotation_x(rotation_angle) * Quat::from_rotation_y(rotation_angle),
                Vec2::new(length, thickness),
                current_color,
            );



        } 


     }



 
                   
  
              
    
}