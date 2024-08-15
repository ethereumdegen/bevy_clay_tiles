

use crate::pre_mesh::extrude_polygon_to_3d;
use crate::pre_mesh::point3_to_array_f32;
use crate::pre_mesh::flatten_indices;
use geo::{Point,LineString,Polygon}; 
use crate::TileMaterialExtension;
 
use crate::tile_material::TileMaterial;
use crate::tiles::ClayTilesRoot;
use crate::tiles_config::ClayTilesConfig;
 
 use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology::TriangleList;


use bevy::ecs::component;
use bevy::prelude::*;


use bevy::pbr::ExtendedMaterial;
/*



This is similar to a 'chunk'
*/



pub(crate) fn clay_tile_block_plugin(app: &mut App) {
    app
    	.add_systems(Update, 

    		(
                build_tile_block_meshes,
                add_needs_rebuild_to_block_mesh, 



                ).chain().run_if(any_with_component::<ClayTileBlock> )



    		)
       // .init_resource::<TileEditDataResource>()
        ;
}




pub type TilePbrBundle = MaterialMeshBundle<TileMaterialExtension>;



//should spatially offset the layer at the appropriate height
#[derive(Component)]
pub struct ClayTileBlock {


    pub polygon_points: Vec<UVec2>


} 


impl ClayTileBlock {


    pub fn points_are_clockwise(&self) -> bool {
        let points = &self.polygon_points;
        let len = points.len();
        let mut sum = 0.0;

        for i in 0..len - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];

            // Convert UVec2 to IVec2 to safely perform arithmetic operations
            let p1_signed = IVec2::new(p1.x as i32, p1.y as i32);
            let p2_signed = IVec2::new(p2.x as i32, p2.y as i32);

            sum += (p2_signed.x - p1_signed.x) as f32 * (p2_signed.y + p1_signed.y) as f32;
        }

        sum <= 0.0
    }

    // Function to ensure points are in counterclockwise order
    pub fn ensure_clockwise(&mut self) {
        if !self.points_are_clockwise() {
            self.polygon_points.reverse();
        }
    }


    pub fn is_complete(&self) -> bool {


        //first point is the same as last point 
        if let Some(first_point) = self.polygon_points.first() {
            if let Some(last_point) = self.polygon_points.last() {
                return first_point == last_point;
            }
        }
        false
    }

    pub fn to_linestring(&self) -> LineString {
 

        let polygon_points_cw = if self.points_are_clockwise() {
            self.polygon_points.clone()  // If CCW, use as is
        } else {
            let mut reversed_points = self.polygon_points.clone(); // Clone and reverse if not CCW
            reversed_points.reverse();
            reversed_points
        };

       let points: Vec<Point> =  polygon_points_cw.iter().map(|p| Point::new(p.x as f64, p.y as f64)).collect();
        LineString::from(points)


    }


    pub fn to_exterior_polygon(&self)  -> Polygon {

         Polygon::new(self.to_linestring(), vec![])
    }


    pub fn build_mesh(&self) -> Option<Mesh> {


       let  polygon = self.to_exterior_polygon();
    
      //  result_polygon.exterior_coords_iter()
 

       let (vertices, indices, uvs) = extrude_polygon_to_3d(  &polygon .into() , 0.2  );

         // Convert vertices to the expected format for Bevy
        let vertex_positions: Vec<[f32; 3]> = vertices.iter().map(point3_to_array_f32).collect();

     
       

        // Flatten indices for Bevy
        let flattened_indices = flatten_indices(&indices);

        let mut mesh = Mesh::new(TriangleList, RenderAssetUsages::default());

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertex_positions,
        );

        // Insert the UV coordinates
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);



        mesh.insert_indices(Indices::U32(flattened_indices));


        Some(mesh)


    }

}


#[derive(Component)]
pub struct ClayTileMesh;


//if this exists, destroy it and rebuild the children w the data 
#[derive(Component)]
pub struct RebuildTileBlock  ;



fn add_needs_rebuild_to_block_mesh(

    mut commands:Commands,

    clay_tile_layer_query: Query<
     (Entity, & ClayTileBlock,  &Parent ), Added<ClayTileBlock>
    >, 

    ){


    for (block_entity, clay_tile_block, parent ) in clay_tile_layer_query.iter(){
        commands.entity(block_entity).insert( RebuildTileBlock );
    }

}


pub fn build_tile_block_meshes(
	mut commands:Commands,
	clay_tile_layer_query: Query<
	 (Entity, & ClayTileBlock,  &Parent ), With<RebuildTileBlock>
	>, 

	 mut meshes: ResMut<Assets<Mesh>>,

    tile_root_query: Query<(&ClayTilesRoot, &ClayTilesConfig)>,
    mut tile_materials: ResMut<Assets<TileMaterialExtension>>,


	){


	for (block_entity, clay_tile_block, parent ) in clay_tile_layer_query.iter(){

        if !clay_tile_block.is_complete() {
            // not complete so we skip 
            continue;
        }



		commands.entity(block_entity).remove::<RebuildTileBlock>();

		
		let Some((clay_tiles_root,clay_tiles_config)) = tile_root_query.get(parent.get()).ok() else {continue};

		//let tile_diffuse_texture = clay_tiles_root.terrain_data_loaded
		let tile_diffuse_texture = clay_tiles_root.get_diffuse_texture_image().clone();
            info!("building clay tile mesh");

	    let tile_material: Handle<TileMaterialExtension> =
                tile_materials.add(ExtendedMaterial {
                    base: StandardMaterial {
                        // can be used in forward or deferred mode.
                       // opaque_render_method: OpaqueRendererMethod::Auto,
                       // alpha_mode: AlphaMode::Mask(0.1),

                        reflectance: 0.2,
                        perceptual_roughness: 0.7,
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
                        diffuse_texture: tile_diffuse_texture.clone(),
                       
                        ..default()
                    },
                });



             let mesh = clay_tile_block.build_mesh();


              let Some( mesh ) = mesh else {continue};
             
            let terrain_mesh_handle = meshes.add(mesh);

            let mesh_bundle = commands
                .spawn(TilePbrBundle {
                    mesh: terrain_mesh_handle,
                    material: tile_material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),

                    ..default()
                })
                .insert(ClayTileMesh)
                .id();

           // chunk_data.material_handle = Some(chunk_terrain_material);

            commands.entity(block_entity)
             .add_child(mesh_bundle);
            
           // chunk_data.chunk_state = ChunkState::FullyBuilt;

            println!("chunk fully built ");

           // commands.entity(entity).despawn();

     }

}