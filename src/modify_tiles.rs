

use crate::tile_edit::TileEditingResource;
use crate::clay_tile_block::ClayTileBlock;
use bevy_mod_raycast::prelude::*;
use bevy::prelude::*;

/*



- be able to modify dimensions -- push and pull them ! 
    For ModifyDragSides...
       When mouse down, use the normal to see if you are clicking the TOP or SIDE, .  THEN  figure out X and Z coords to figure out which bottom  segment you are grabbing . 



*/

pub(crate) fn modify_tiles_plugin(app: &mut App) {
    app .init_resource::<ModifyTileResource>()
    	.add_systems(Update, 

    		(
                raycast_to_select_tiles,

                deselect_tiles, 

             
                //add_selectable_to_clay_tile_children, 
               // add_gizmo_component_to_selected_tile,

            
                ).chain() 



    		);

    }


#[derive(Resource ,Default)]
pub struct ModifyTileResource {

    pub modifying_tile: Option<Entity>,
    pub modifying_side: Option<TileBlockFaceType>,
    pub modifying_segment_index: Option<usize>

}

#[derive(Clone,Hash,Eq,PartialEq,Debug)]
pub enum TileBlockFaceType {

	Top,
	Bottom,
	Side, 

} 

#[derive(Component)]
pub struct TileHeightEditGizmo ;


#[derive(Component)]
pub struct ClayTileBlockSelectable; 


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

    mut raycast: Raycast,
    cursor_ray: Res<CursorRay>,

    raycast_filter_query: Query<Entity, With<ClayTileBlockSelectable>>,  //make sure meshes have this ?
    mouse_input: Res<ButtonInput<MouseButton>>,

     tile_edit_resource: Res<TileEditingResource>,

       mut modify_tile_resource: ResMut<ModifyTileResource>,
    ){


        let just_clicked = mouse_input.just_pressed(MouseButton::Left);
 
        if !just_clicked {return};

        if !tile_edit_resource.able_to_select_tiles() {return} ;

      let filter = |entity| raycast_filter_query.contains(entity);
      if let Some(cursor_ray) = **cursor_ray {

       let hits = raycast.cast_ray(cursor_ray, &RaycastSettings::default().with_filter(&filter));

       if let Some((first_hit_entity,  intersection_data)) = hits.first(){

            info!("selecting tile {:?}",  intersection_data);

            modify_tile_resource.modifying_tile = Some(*first_hit_entity);


            // if intesect w Side,   use X and Z to pick the BEST segment index as possible ... 
            

       }

    }
}


fn deselect_tiles(
	mouse_input: Res<ButtonInput<MouseButton>>,
 

      mut modify_tile_resource: ResMut<ModifyTileResource>,
){


	   let just_released = mouse_input.just_released(MouseButton::Left);
 
        if !just_released {return};

          modify_tile_resource.modifying_tile = None; 
           




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