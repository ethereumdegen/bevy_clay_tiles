

use crate::tile_edit::TileEditingResource;
use crate::clay_tile_block::ClayTileBlock;
//use bevy_mod_raycast::prelude::*;
use bevy::prelude::*;

use bevy::picking::backend::ray::RayMap; 

pub(crate) fn tile_gizmos_plugin(app: &mut App) {
    app
    	.add_systems(Update, 

    		(
                raycast_to_select_tiles,

             
                //add_selectable_to_clay_tile_children, 
                add_gizmo_component_to_selected_tile,

            
                ).chain() 



    		);

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

     mut raycast: MeshRayCast,
    //mut raycast: Raycast,
    cursor_ray: Res<RayMap>,

    raycast_filter_query: Query<Entity, With<ClayTileBlockSelectable>>,  //make sure meshes have this ?
    mouse_input: Res<ButtonInput<MouseButton>>,

    mut tile_edit_resource: ResMut<TileEditingResource>,
    ){


        let just_clicked = mouse_input.just_pressed(MouseButton::Left);
 
        if !just_clicked {return};

        if !tile_edit_resource.able_to_select_tiles() {return} ;

      let filter = |entity| raycast_filter_query.contains(entity);
     // if let Some(cursor_ray) = **cursor_ray {
       for (_, cursor_ray) in cursor_ray.iter() {


       let hits = raycast.cast_ray(cursor_ray, &RaycastSettings::default().with_filter(&filter));

       if let Some((first_hit_entity,  intersection_data)) = hits.first(){

            info!("selecting tile {:?}",  intersection_data);

            tile_edit_resource.selected_tile = Some(*first_hit_entity);
            

       }

    }
}





// need to render a gizmo on the selected tile

fn add_gizmo_component_to_selected_tile(
    mut commands: Commands, 
    tile_edit_resource: Res <TileEditingResource>,
 ){


    let Some(selected_tile) = &tile_edit_resource.selected_tile  else {return};





   // info!("render gizmo on tile ");

}