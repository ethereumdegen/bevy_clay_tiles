

use crate::ClayTilesTypesConfigResource;
use crate::tile_types_config::TileTypeConfig;
use crate::tile_gizmos::ClayTileBlockSelectable;
use crate::tile_types_config;
//use crate::tiles_texturing::ClayTilesTypesConfigResource;
use bevy::render::render_resource::Origin3d;
use bevy_material_tool::material_overrides::MaterialOverrideComponent;
use serde::Serialize;
use serde::Deserialize;
//use crate::ClayTilesTexturingResource;
use crate::pre_mesh::PreMesh;
use crate::tile_edit::TileEditingResource;
 
use bevy::color::palettes::tailwind;
use geo::{Point,LineString,Polygon}; 
use crate::TileMaterialExtension;
 
use crate::tile_material::TileMaterial;
 
 
  
 
 
use bevy::prelude::*;


use bevy::pbr::ExtendedMaterial;
/*



This is similar to a 'chunk'
*/



pub(crate) fn clay_tile_block_plugin(app: &mut App) {
    app


        .register_type::<ClayTileBlock>()
    	.add_systems(Update, 

    		(
                build_tile_block_meshes,
                init_build_clay_tile_block, 
            
                ).chain().run_if(any_with_component::<ClayTileBlock> )



    		)
       // .init_resource::<TileEditDataResource>()
        


        .add_systems(Update, 

            (
               

                render_gizmos_for_clay_tile_block_builders, 

                update_clay_tile_block_builders,



                ).chain().run_if(any_with_component::<ClayTileBlockBuilder> )



            )
       // .init_resource::<TileEditDataResource>()
        ;

}







#[derive(Component)]
pub struct ClayTileBlockBuilder {


    pub polygon_points: Vec<IVec2>,

     pub height_level: u32, 
     pub tile_type_index: usize,
     pub mesh_height: f32, 


}

impl Default for ClayTileBlockBuilder {



fn default() -> Self {

    Self{
        polygon_points: Vec::new(),
        height_level: 0,
        tile_type_index: 0,
        mesh_height: 0.2
    }

 }
}




impl ClayTileBlockBuilder {


    pub fn get_origin_point(&self) -> Option<&IVec2> {

        self.polygon_points.first()

    }


    pub fn points_are_counterclockwise(&self) -> bool {
        let points = &self.polygon_points;
        let len = points.len();
        let mut sum = 0.0;

        for i in 0..len - 1 {
            let p1_signed = points[i];
            let p2_signed = points[i + 1];

            // Convert UVec2 to IVec2 to safely perform arithmetic operations
           // let p1_signed = IVec2::new(p1.x , p1.y );
        //  let p2_signed = IVec2::new(p2.x  , p2.y  );
 
            sum += (p2_signed.x - p1_signed.x) as f32 * (p2_signed.y + p1_signed.y) as f32;
        }

        sum > 0.0
    }

    // Function to ensure points are in counterclockwise order
    pub fn ensure_counterclockwise(&mut self) {
        if !self.points_are_counterclockwise() {
            self.polygon_points.reverse();
        }
    }


    pub fn is_complete(&self) -> bool {


        if self.polygon_points.len() < 3 {
            return false ;
        }

        //first point is the same as last point 
        if let Some(first_point) = self.polygon_points.first() {
            if let Some(last_point) = self.polygon_points.last() {
                return first_point == last_point;
            }
        }
        false
    }


     pub fn build(&self) -> Option<ClayTileBlock> {

        if !self.is_complete(){

            return None ;
        }

        let polygon_points_ccw = if self.points_are_counterclockwise() {
            self.polygon_points.clone()  // If CCW, use as is
        } else {
            let mut reversed_points = self.polygon_points.clone(); // Clone and reverse if not CCW
            reversed_points.reverse();
            reversed_points
        };


        Some(
            ClayTileBlock {

                polygon_points: polygon_points_ccw,
                height_level: self.height_level.clone(), 
                tile_type_index: self.tile_type_index.clone() ,
                mesh_height: self.mesh_height.clone(), 
              //  uv_expansion_factor : 0.25, // for now 

                ..default()
            }
        )
     }



}



fn render_gizmos_for_clay_tile_block_builders(
    mut gizmos: Gizmos,
    query: Query<&ClayTileBlockBuilder>,

      tile_edit_resource: Res<TileEditingResource>,

) {

   let height_offset  = tile_edit_resource.get_build_layer_height() as f32;

    for builder in query.iter() {
        let points = &builder.polygon_points;

        // Render gizmo points
        for &point in points.iter() {
            let position = Vec3::new(point.x as f32, 1.0 * height_offset, point.y as f32);

            let radius = 0.1;
            let rotation = Quat::IDENTITY;
            let color : Color = tailwind::EMERALD_400.into() ;

            gizmos.sphere(position, rotation, radius, color) ;
           
        }

        // Render gizmo lines between points
        for i in 0..points.len() - 1 {
            let start = Vec3::new(points[i].x as f32, 1.0 * height_offset, points[i].y as f32);
            let end = Vec3::new(points[i + 1].x as f32, 1.0 * height_offset, points[i + 1].y as f32);

            let color:  Color = tailwind::AMBER_400.into();

            gizmos.line(start, end, color)
            
        }
    }
}




fn update_clay_tile_block_builders(
    mut commands: Commands,
    query: Query<(Entity, &ClayTileBlockBuilder, Option<&Parent>)>,

     tile_edit_resource: Res<TileEditingResource>,
) {
    for (entity, builder, parent_option) in query.iter() {
        if builder.is_complete() {

            let new_tile_parent_entity = tile_edit_resource.get_new_tile_parent_entity();

            // Build the ClayTileBlock from the builder
            if let Some(clay_tile_block) = builder.build() {
                // Despawn the builder entity
                commands.entity(entity).despawn_recursive();

                // Spawn the new ClayTileBlock
               let new_block =  commands.spawn(SpatialBundle::default())
               
                .insert( clay_tile_block )
                //.insert( RebuildTileBlock ) 

                .id() ;

              
                if let Some(parent) = new_tile_parent_entity{ 
                    commands.entity(new_block)
                      .set_parent( parent ) ;
                  }
            }
        }
    }
}




fn init_build_clay_tile_block (
    mut commands: Commands,
    tile_block_query: Query< Entity ,( With<ClayTileBlock>,  Without<ClayTileMesh> ) >,
) {
   
    for  entity   in tile_block_query.iter() {
        if let Some(mut cmd) = commands.get_entity(entity){ 
            info!("insert rebuild tile block ");
            cmd.insert(RebuildTileBlock); 
        }
    }
}





pub type TilePbrBundle = MaterialMeshBundle<TileMaterialExtension>;



//should spatially offset the layer at the appropriate height
#[derive(Component,Clone,Serialize,Deserialize,Reflect,Debug)]
#[reflect(Component)]
pub struct ClayTileBlock {

        //should always be counterclockwise ! 
    pub polygon_points: Vec<IVec2>,

    pub mesh_height: f32,  // 0.2 default

    pub mesh_bevel_factor: f32, //0  default 

    pub height_level: u32, 

  //  pub uv_expansion_factor: f32 ,

    pub tile_type_index: usize,  

} 

impl Default for ClayTileBlock { 
 
    fn default() -> Self {  

        Self {
            polygon_points: Vec::new(),
            mesh_height: 0.2,
            mesh_bevel_factor: 0.0,
            height_level : 0 ,
         //   uv_expansion_factor: 1.0,
            tile_type_index: 0 
        }
     }

}


impl ClayTileBlock {


    pub fn get_origin_point(&self) -> Option<&IVec2> {

        self.polygon_points.first()

    }

    pub fn to_linestring(&self) -> LineString {
 

        
       let points: Vec<Point> =  self.polygon_points.iter().map(|p| Point::new(p.x as f64, p.y as f64)).collect();
        LineString::from(points)


    }


    pub fn is_complete(&self) -> bool {



        if self.polygon_points.len() < 3 {
            return false ;
        }

        //first point is the same as last point 
        if let Some(first_point) = self.polygon_points.first() {
            if let Some(last_point) = self.polygon_points.last() {
                return first_point == last_point;
            }
        }
        false
    }




    pub fn to_exterior_polygon(&self)  -> Polygon {

         Polygon::new(self.to_linestring(), vec![])
    }


    pub fn build_mesh(&self) -> Option<Mesh> {


       let  polygon = self.to_exterior_polygon();
    
      //  result_polygon.exterior_coords_iter()


       let mesh_height_scale = & self.mesh_height ;
        let mesh_bevel_factor = &self.mesh_bevel_factor ;

        let origin_offset = self.get_origin_point().map( |p| 

            *p * -1

         ).unwrap_or( IVec2::new(0,0) ) ;
 
       let pre_mesh = PreMesh::extrude_2d_polygon_to_3d(
        &polygon .into() , 
        origin_offset,
        *mesh_height_scale as f64,
        *mesh_bevel_factor as f64

        ) ?;
       let mut mesh = pre_mesh
       .build() 
       ;

       /*let generated_tangents = mesh.generate_tangents()  ;

       match generated_tangents {
        Ok(_) => {},
        Err(error) => warn!("Could not generate mesh tangents {:?}", error)

       };*/

        Some(mesh)


    }

}


#[derive(Component,Default)]
pub struct ClayTileMesh;


//if this exists, destroy it and rebuild the children w the data 
#[derive(Component,Default)]
pub struct RebuildTileBlock  ;


/*
fn add_needs_rebuild_to_block_mesh(

    mut commands:Commands,

    clay_tile_layer_query: Query<
     (Entity, & ClayTileBlock,  &Parent ), Added<ClayTileBlock>
    >, 

    ){


    for (block_entity, clay_tile_block, parent ) in clay_tile_layer_query.iter(){
        commands.entity(block_entity).insert( RebuildTileBlock );
    }

}*/


pub fn build_tile_block_meshes(
	mut commands:Commands,
	mut clay_tile_layer_query: Query<
	 (Entity, & ClayTileBlock,  Option<&Parent>, &mut Transform ), With<RebuildTileBlock>
	>, 

	 mut meshes: ResMut<Assets<Mesh>>,

      mut materials: ResMut<Assets<StandardMaterial>>,
    //tile_texture_resource: Res <ClayTilesTexturingResource>,
  
    mut tile_materials: ResMut<Assets<TileMaterialExtension>>,

    tile_types_config: Res<ClayTilesTypesConfigResource>,


	){


	for (block_entity, clay_tile_block, parent_option, mut tile_block_transform ) in clay_tile_layer_query.iter_mut(){

        if !clay_tile_block.is_complete() {
            // not complete so we skip 

              warn!("Tile segments are incomplete");
            continue;
        }

 


		commands.entity(block_entity).remove::<RebuildTileBlock>();

		
		/*let Some((clay_tiles_root,clay_tiles_config)) = tile_root_query.get(parent.get()).ok() else {
            warn!("Invalid tile parent");
            continue
        };*/

		//let tile_diffuse_texture = clay_tiles_root.terrain_data_loaded
	//	let tile_diffuse_texture = tile_texture_resource.get_diffuse_texture_image().clone();
     //   let tile_normal_texture = tile_texture_resource.get_normal_texture_image().clone();
        
        info!("building clay tile mesh");

        let tile_type_id = clay_tile_block.tile_type_index;


        
        
        let tile_type_config = tile_types_config.tile_type_data.get( &tile_type_id ) 
        .expect("unable to load tile types config");
       
            // get uv exp factor from tile_types_config 

        let color_texture_expansion_factor = &tile_type_config.diffuse_uv_expansion_factor;
        let diffuse_color_tint = &tile_type_config.diffuse_color_tint.unwrap_or(LinearRgba::rgb(1.0, 1.0, 1.0));
        let tile_material_name = &tile_type_config.material_name;

	    /*let tile_material: Handle<TileMaterialExtension> =
                tile_materials.add(ExtendedMaterial {
                    base: StandardMaterial {
                        // can be used in forward or deferred mode.
                       // opaque_render_method: OpaqueRendererMethod::Auto,
                       // alpha_mode: AlphaMode::Mask(0.1),
                        
                     //   unlit: true,   

                        reflectance: 0.05,
                        perceptual_roughness: 0.85,

                    //    specular_transmission: 0.1, //kills the depth buffer

                        // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                        // in forward mode, the output can also be modified after lighting is applied.
                        // see the fragment shader `extended_material.wgsl` for more info.
                        // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                        // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                        ..Default::default()
                    },
                    extension: TileMaterial {
                       /* chunk_uniforms: ChunkMaterialUniforms {
                            color_texture_expansion_factor , //why wont this apply to shader properly ?
                            chunk_uv,
                        },*/
                     //   tool_preview_uniforms: ToolPreviewUniforms::default(),

                        color_texture_expansion_factor:*color_texture_expansion_factor ,
                        diffuse_texture: tile_diffuse_texture.clone(),
                        diffuse_color_tint: diffuse_color_tint.to_vec4(),

                        normal_texture: tile_normal_texture.clone(),

                        tile_texture_index: *tile_diffuse_texture_index,
                       
                        ..default()
                    },
                });*/


            if let Some(origin_point) =  clay_tile_block.get_origin_point() {
                tile_block_transform.translation.x = origin_point.x as f32;
                tile_block_transform.translation.z = origin_point.y as f32;
            }
           
             tile_block_transform.translation.y = clay_tile_block.height_level as f32;

             let mesh = clay_tile_block.build_mesh();


              let Some( mesh ) = mesh else {
                warn!("could not build mesH!!");  //remove entity ? 
                continue
            };


            commands.entity(block_entity).despawn_descendants();


             
            let terrain_mesh_handle = meshes.add(mesh);

           /* let mesh_bundle = commands
                .spawn(TilePbrBundle {
                    mesh: terrain_mesh_handle,
                    material: tile_material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),

                    ..default()
                })
                .insert(ClayTileMesh)
                .insert( ClayTileBlockSelectable )
                .id();



           // chunk_data.material_handle = Some(chunk_terrain_material);

            commands.entity(block_entity)
             .add_child(mesh_bundle);   

    */


        let simple_material =  materials.add( StandardMaterial::from_color( Color::BLACK ) );

             commands.entity(block_entity)
             .insert(  (
                terrain_mesh_handle,
                    
                    ClayTileMesh,
                    ClayTileBlockSelectable,
                    Name::new("ClayTileBlock"),
                    simple_material,
                    MaterialOverrideComponent{material_override: tile_material_name.to_string()}

                )  )

             ;
            
           // chunk_data.chunk_state = ChunkState::FullyBuilt;

            println!("chunk fully built ");

           // commands.entity(entity).despawn();

     }

}
