

use crate::tile_edit::ModifyTileTool;
use crate::{clay_tile_block::RebuildTileBlock, tile_edit::{EditingTool, TileEditingResource}};

  use bevy::platform::collections::hash_map::HashMap;

 use bevy::platform::collections::hash_set::HashSet;

use crate::clay_tile_block::ClayTileBlock;
//use bevy::reflect::List;
//use bevy_mod_raycast::prelude::*;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;


use bevy::picking::backend::ray::RayMap; 
/*



- be able to modify dimensions -- push and pull them ! 
    For ModifyDragSides...
       When mouse down, use the normal to see if you are clicking the TOP or SIDE, .  THEN  figure out X and Z coords to figure out which bottom  segment you are grabbing . 


	when mouse moves, just take the two points of the segment that is 'selected' and translate those points 

- if a point is selected, translate that point only  ! 


*/

pub(crate) fn modify_tiles_plugin(app: &mut App) {
    app .init_resource::<ModifyTileResource>()
    	.add_systems(Update, 

    		(     

                update_inputs,
                raycast_to_select_tiles,

                deselect_tiles, 

                update_modify_points, 
                update_modify_mesh_height,


                handle_apply_modifications,

                

             
                //add_selectable_to_clay_tile_children, 
                // add_gizmo_component_to_selected_tile,

            
                ).chain() 



    		);

    }


#[derive(Resource ,Default)]
pub struct ModifyTileResource { 

    pub lock_to_single_axis: bool,

    pub modifying_tile: Option<Entity>,
    pub modifying_side: Option<TileBlockFaceType>,
   // pub modifying_segment_index: Option<usize>,

    pub modifying_point_indices: Option<HashSet<usize>>,


    pub modify_origin_point: Option<Vec3>,
    pub modify_cursor_origin_coords: Option<Vec2>,


    pub modify_drag_mode : Option<ModifyDragMode>


}



#[derive(Clone,Hash,Eq,PartialEq,Debug)]
pub enum ModifyDragMode{

    MeshHeight,
    BaseHeightLevel,
    PointIndices,

}

#[derive(Clone,Debug,Component)]
pub struct ClayTileBlockPointsTranslation {

  pub point_translations: HashMap<usize, IVec2>
}


#[derive(Clone,Debug,Component)]
pub struct ClayTileBlockMeshHeightTranslation {

    pub mesh_height_delta: i32 
}

#[derive(Clone,Debug,Component)]
pub struct ModifyingClayTile; 



#[derive(Clone,Debug,Component)]
pub struct ApplyModifications; 

#[derive(Clone,Debug)]
pub struct LineSegment {

	pub start_point: IVec2,
	pub end_point: IVec2,

}

#[derive(Clone,Hash,Eq,PartialEq,Debug)]
pub enum TileBlockFaceType {

	Top,
	Bottom,
	Side, 

} 

impl TileBlockFaceType {


    fn estimate_from_normal(normal: Vec3) -> Self {
    // Identify the axis with the maximum absolute value in the normal vector
        let dominant_axis = if normal.x.abs() > normal.y.abs() && normal.x.abs() > normal.z.abs() {
            "x"
        } else if normal.y.abs() > normal.x.abs() && normal.y.abs() > normal.z.abs() {
            "y"
        } else {
            "z"
        };

        // Map the dominant axis to the corresponding face type
        match dominant_axis {
            "x" | "z" => TileBlockFaceType::Side,
            "y" if normal.y > 0.0 => TileBlockFaceType::Top,
            "y" if normal.y < 0.0 => TileBlockFaceType::Bottom,
            _ => TileBlockFaceType::Side,  // Fallback case, though shouldn't be reached
        }
    }


}

#[derive(Component)]
pub struct TileHeightEditGizmo ;


#[derive(Component)]
pub struct ClayTileBlockSelectable; 



fn update_inputs(



     key_inputs: Res<ButtonInput<KeyCode>>,
     mut modify_tile_resource: ResMut<ModifyTileResource>,


){

    let shift_is_pressed = key_inputs.pressed(KeyCode::ShiftLeft);
    modify_tile_resource.lock_to_single_axis = shift_is_pressed;


}
/*
fn add_selectable_to_clay_tile_children(

	mut commands: Commands,
	tile_block_query: Query<Entity, With<ClayTileBlock>>,

	children_query: Query<&Children>,


	   tile_edit_resource: Res <TileEditingResource>,

	){


	 if !tile_edit_resource.able_to_select_tiles() {return} ;

   for tile_block_entity in tile_block_query.iter(){

   	 		for child in DescendantIter::new(&children_query,  tile_block_entity) {
                   if let Some(mut cmd) = commands.get_entity(child) {

                   	   cmd.insert( ClayTileBlockSelectable );


                   }
         }

   }
   

}*/


 
fn raycast_to_select_tiles(
    mut commands:Commands,
  

    //mut raycast: Raycast,
    mut raycast: MeshRayCast,

    cursor_ray: Res<RayMap>,

    raycast_filter_query: Query<Entity, With<ClayTileBlockSelectable>>,  //make sure meshes have this ?
    mouse_input: Res<ButtonInput<MouseButton>>,

    clay_tile_block_query: Query<&ClayTileBlock>,

    tile_edit_resource: Res<TileEditingResource>,

    mut modify_tile_resource: ResMut<ModifyTileResource>,


   window_query: Query<&Window, With<PrimaryWindow>>,
 


    ){
 
        let just_clicked = mouse_input.just_pressed(MouseButton::Left);
 
        if !just_clicked {return};

        let Some(cursor_position) = window_query.single().ok().map( |w| w.cursor_position() ).flatten()  else {return} ;



        if !tile_edit_resource.able_to_select_tiles() {return} ;

          
      let filter = |entity| raycast_filter_query.contains(entity);
    //  if let Some(cursor_ray) = **cursor_ray {
     for   (_, cursor_ray)  in cursor_ray.iter() {

       let hits = raycast.cast_ray(*cursor_ray, &MeshRayCastSettings::default().with_filter(&filter));

            //no hits if the mesh is dragged inverted ! 


       if let Some((first_hit_entity,  intersection_data)) = hits.first(){

            info!("selecting tile {:?}",  intersection_data);

            modify_tile_resource.modifying_tile = Some(*first_hit_entity);


            if let Some( clay_tile_block ) = clay_tile_block_query.get( *first_hit_entity).ok() {

                if let Some(mut cmd) = commands.get_entity(*first_hit_entity).ok() {
                    cmd.insert(ModifyingClayTile);
                }

            	let mut base_segments: Vec< LineSegment > = Vec::new();
            	let tile_block_polygon_points = &clay_tile_block.polygon_points;

            	let clay_tile_height_level = &clay_tile_block.height_level; 


            	let intersection_position = intersection_data.point ;
            	let intersection_normal = intersection_data.normal ; 

                let face_type = TileBlockFaceType::estimate_from_normal(intersection_normal);



                if tile_edit_resource.get_selected_tool().as_ref().is_some_and(|t| t == &EditingTool::ModifyTile(ModifyTileTool::ModifyDragSides)) {

                     if face_type == TileBlockFaceType::Side {


                        for point_index in 0..tile_block_polygon_points.len() {

                            let start_point = tile_block_polygon_points[(point_index + 0) % tile_block_polygon_points.len()] ;
                            let end_point = tile_block_polygon_points[(point_index + 1) % tile_block_polygon_points.len()] ;

                            base_segments.push(  
                                LineSegment{
                                    start_point,
                                    end_point

                                }
                             );
                        }   


                           // Find the segment with the minimum distance to the intersection position
                        let mut closest_segment = None;
                        let mut min_distance = f32::MAX;

                        for (index, segment) in base_segments.iter().enumerate() {
                            let start_ivec = IVec3::new( segment.start_point.x, *clay_tile_height_level as i32, segment.start_point.y);
                            let end_ivec = IVec3::new( segment.end_point.x, *clay_tile_height_level as i32, segment.end_point.y);

                            let start: Vec3 = Vec3::new( start_ivec.x as f32, start_ivec.y as f32, start_ivec.z as f32 );
                            let end: Vec3 = Vec3::new( end_ivec.x as f32, end_ivec.y as f32, end_ivec.z as f32 );

                            // Project intersection_position onto the line segment
                            let segment_vector = end - start;
                            let to_intersection = intersection_position - start;
                            let projection_length = to_intersection.dot(segment_vector) / segment_vector.length_squared();
                            let projection = start + projection_length.clamp(0.0, 1.0) * segment_vector;

                            // Calculate the distance from the intersection position to the projection
                            let distance = (intersection_position - projection).length();

                            if distance < min_distance {
                                min_distance = distance;
                                closest_segment = Some(index);
                            }
                        }

                        if let Some(best_segment_index) = closest_segment {
                            info!("segment index: {}", best_segment_index);

                            let first_index = 0;
                            let last_index = tile_block_polygon_points.len()-1 ;

                            let start_index = (best_segment_index + 0) % tile_block_polygon_points.len();
                            let end_index = (best_segment_index + 1) % tile_block_polygon_points.len();
                            
                            let mut point_indices = HashSet::with_hasher(Default::default() ) ;

                            point_indices.insert(start_index);
                            point_indices.insert(end_index);

                              //a fix so the first and last vertices always move together
                            if start_index == first_index {
                                point_indices.insert(last_index);
                            }

                            if end_index == last_index {
                                point_indices.insert(first_index);
                            }


                        //    modify_tile_resource.modifying_segment_index = Some(best_segment_index);
                            modify_tile_resource.modify_origin_point = Some( intersection_position );
                            modify_tile_resource.modify_cursor_origin_coords = Some(
                                cursor_position
                                );

                            modify_tile_resource.modifying_point_indices = Some(
                                point_indices
                            );

                             modify_tile_resource.modify_drag_mode = Some(ModifyDragMode::PointIndices );

                            // You can now do something with the best segment index
                            // For example, you can store it in modify_tile_resource or use it in another function
                        }



                    }

                     if face_type == TileBlockFaceType::Top {


                         modify_tile_resource.modify_origin_point = Some( intersection_position );
                         modify_tile_resource.modify_drag_mode = Some( ModifyDragMode::MeshHeight ) ;
                         modify_tile_resource.modify_cursor_origin_coords = Some(
                                cursor_position
                                );
                     }


                     if face_type == TileBlockFaceType::Bottom {


                         modify_tile_resource.modify_origin_point = Some( intersection_position );
                         modify_tile_resource.modify_drag_mode = Some( ModifyDragMode::BaseHeightLevel ) ;
                         modify_tile_resource.modify_cursor_origin_coords = Some(
                                cursor_position
                                );
                     }



                }
               
                if tile_edit_resource.get_selected_tool().as_ref().is_some_and(|t| t == &EditingTool::ModifyTile(ModifyTileTool::ModifyDragVertices)) {


                     if face_type == TileBlockFaceType::Side {

                        // Find the closest vertex to the intersection position
                        let mut closest_vertex_index = None;
                        let mut min_distance = f32::MAX;

                        for (index, point) in tile_block_polygon_points.iter().enumerate() {

                           // let Some(point) = point.downcast_ref::<IVec2>() else {continue};
                               

                            let vertex_position = Vec3::new(point.x as f32, *clay_tile_height_level as f32, point.y as f32);

                            // Calculate the distance from the intersection position to the vertex
                            let distance = (intersection_position - vertex_position).length();

                            if distance < min_distance {
                                min_distance = distance;
                                closest_vertex_index = Some(index);
                            }
                        }

                        if let Some(best_vertex_index) = closest_vertex_index {



                            let first_index = 0;
                            let last_index = tile_block_polygon_points.len()-1 ;


                            info!("vertex index: {}", best_vertex_index);

                            let mut point_indices =  HashSet::with_hasher(Default::default() ) ;
                            point_indices.insert(best_vertex_index);


                            //a fix so the first and last vertices always move together
                            if best_vertex_index == first_index {
                                point_indices.insert(last_index);
                            }

                            if best_vertex_index == last_index {
                                point_indices.insert(first_index);
                            }





                           // modify_tile_resource.modifying_vertex_index = Some(best_vertex_index);
                            modify_tile_resource.modify_origin_point = Some(intersection_position);
                            modify_tile_resource.modifying_point_indices = Some(point_indices);
                            modify_tile_resource.modify_drag_mode = Some( ModifyDragMode::PointIndices )  ;
                            modify_tile_resource.modify_cursor_origin_coords = Some(
                                cursor_position
                                );
                            // You can now modify the vertex position based on further user input, like dragging
                        }
                    }


                }
            



            }


             

       }

    }
}


fn deselect_tiles(
    mut commands:Commands, 
	mouse_input: Res<ButtonInput<MouseButton>>, 
    mut modify_tile_resource: ResMut<ModifyTileResource>,

    modifying_block_query: Query<Entity, With<ModifyingClayTile>>
){


	   let just_released = mouse_input.just_released(MouseButton::Left);
 
        if !just_released {return}; 

        if let Some( tile_entity ) = &modify_tile_resource.modifying_tile {

            commands.entity(*tile_entity).insert(ApplyModifications);
        }

        for block_entity in modifying_block_query.iter(){

            commands.get_entity(block_entity)
            .map(|mut cmd| {  cmd.remove::<ModifyingClayTile>(); });   
          
        }


        modify_tile_resource.modifying_tile = None; 
         // modify_tile_resource.modifying_segment_index = None;

        modify_tile_resource.modify_origin_point = None;
        modify_tile_resource.modify_cursor_origin_coords = None;
        modify_tile_resource.modifying_point_indices = None;

        modify_tile_resource.modify_drag_mode = None;
           




}


//this uses worldspace coords 

fn update_modify_points(


     mut commands: Commands,

     //  clay_tile_block_query: Query<&  ClayTileBlock>,

       modify_tile_resource: Res<ModifyTileResource>,

     cursor_ray: Res<RayMap>,





){

    if let Some( clay_tile_entity ) = &modify_tile_resource.modifying_tile {


        if modify_tile_resource.modify_drag_mode.as_ref() != Some( &ModifyDragMode::PointIndices ){
            return;
        }



        let Some(modify_current_drag_startpoint) = &modify_tile_resource.modify_origin_point else {return};

        let modify_level_height = modify_current_drag_startpoint.y.clone() ;


        let mut modify_current_drag_endpoint = None ;

        for (_ , cursor_ray)   in cursor_ray.iter() {
                let origin = &cursor_ray.origin; 
                let direction = &cursor_ray.direction;

              if direction.y.abs() > 1e-6 {  // Ensure we're not dividing by zero
                    let t = (modify_level_height - origin.y) / direction.y; 
            

                    modify_current_drag_endpoint = Some( *origin + *direction * t ); 
              }
          }

        let Some( modify_current_drag_endpoint  ) = modify_current_drag_endpoint else {return};



        let drag_delta = modify_current_drag_endpoint - *modify_current_drag_startpoint ;

        let mut drag_delta_ivec:IVec2 = IVec2::new( drag_delta.x as i32,  drag_delta.z as i32  );
        info!("drag_delta , {:?}", drag_delta);



        let lock_to_single_axis = &modify_tile_resource.lock_to_single_axis ;

        if *lock_to_single_axis {

            if drag_delta_ivec.x.abs() > drag_delta_ivec.y.abs() {

                drag_delta_ivec.y = 0;
            }else {

                drag_delta_ivec.x = 0;
            }

        }

       // info!("lock_to_single_axis {:?}, {:?}", lock_to_single_axis, drag_delta_ivec);


        let Some( modifying_point_indices ) =  &modify_tile_resource.modifying_point_indices else {return};

        let mut point_translations = HashMap::with_hasher(Default::default() ) ;

        for i in modifying_point_indices.iter(){

             point_translations.insert( *i , drag_delta_ivec.clone() );

        }
       


        if let Some(mut cmd) = commands.get_entity(*clay_tile_entity ).ok() {
            info!("modifying tile..");

            cmd
            .insert(  ClayTileBlockPointsTranslation {

                point_translations  


            }  )
           .insert( RebuildTileBlock )
            ;
        }

    }




}



// this actually uses screenspace coords 

fn update_modify_mesh_height(


     mut commands: Commands,

      modify_tile_resource: Res<ModifyTileResource>,

   //  cursor_ray: Res<CursorRay>,

      window_query: Query<&Window, With<PrimaryWindow>>,
 





){

  

    if let Some( clay_tile_entity ) = &modify_tile_resource.modifying_tile {


        if modify_tile_resource.modify_drag_mode.as_ref() != Some( &ModifyDragMode::MeshHeight ){
            return;
        }


     let Some(modify_current_drag_startpoint) = &modify_tile_resource.modify_cursor_origin_coords else {return};


    let Some(current_cursor_position) = window_query.single().ok().map( |w| w.cursor_position()).flatten()  else {return} ;


       
        let drag_delta =  modify_current_drag_startpoint.y - current_cursor_position.y   ;

        let sensitivity = 0.02;

       
        let    drag_delta_int:i32 =  ( drag_delta * sensitivity) as i32 ;
        info!("drag_delta , {:?}", drag_delta);

        


        if let Some(mut cmd) = commands.get_entity(*clay_tile_entity ).ok() {
            info!("modifying tile..");

            cmd
            .insert(  ClayTileBlockMeshHeightTranslation {

                mesh_height_delta: drag_delta_int  


            }  )
           .insert( RebuildTileBlock )
            ;
        }

    }




}



fn handle_apply_modifications(

    mut commands:Commands, 
      clay_tile_query: Query<(Entity, &  ClayTileBlock, Option<&ClayTileBlockPointsTranslation>, Option<&ClayTileBlockMeshHeightTranslation>), With<ApplyModifications>>



){


for (clay_tile_entity, clay_tile_block, point_translation_comp, mesh_height_translation_comp) in clay_tile_query.iter(){


    let mut updated_clay_tile_block = clay_tile_block.clone();

    



     if let Some(point_translation_comp) = point_translation_comp {

           let polygon_points:&mut Vec<IVec2> = &mut updated_clay_tile_block.polygon_points;


            let translations = &point_translation_comp.point_translations;

            // Apply translations to the polygon points
            for (point_index, translation) in translations.iter() {


                polygon_points[*point_index] += *translation;
                 
            }


            commands.entity(clay_tile_entity).remove::<ClayTileBlockPointsTranslation>();
     
    }

     if let Some(mesh_height_translation_comp) = mesh_height_translation_comp {
      
        let   delta_height = &mesh_height_translation_comp.mesh_height_delta;

        

        updated_clay_tile_block.mesh_height += *delta_height as f32 ;

         commands.entity(clay_tile_entity).remove::<ClayTileBlockMeshHeightTranslation>();
    }





       
        commands.entity(clay_tile_entity).remove::<ApplyModifications>();

      if updated_clay_tile_block.is_complete() {

        commands.entity(clay_tile_entity).insert( updated_clay_tile_block  );
      //  commands.entity(clay_tile_entity).insert(RebuildTileBlock);
      }

      commands.entity(clay_tile_entity).insert(RebuildTileBlock);
   
}



}

// need to render a gizmo on the selected tile
/*
fn add_gizmo_component_to_selected_tile(
    mut commands: Commands, 
    tile_edit_resource: Res <TileEditingResource>,
 ){


    let Some(selected_tile) = &tile_edit_resource.modifying_tile  else {return};





   // info!("render gizmo on tile ");

}*/