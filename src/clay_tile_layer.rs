

use crate::pre_mesh::build_mesh_from_operations;
use crate::tile_material::TileMaterial;
use crate::tiles::ClayTilesRoot;
use crate::tiles_config::ClayTilesConfig;
use crate::{clay_tile_operation::ClayTileOperation, TileMaterialExtension};
use bevy::ecs::component;
use bevy::prelude::*;


use bevy::pbr::ExtendedMaterial;
/*



This is similar to a 'chunk'
*/



pub(crate) fn clay_tile_layer_plugin(app: &mut App) {
    app
    	.add_systems(Update, 

    		(build_tile_layer_meshes).chain().run_if(any_with_component::<ClayTileLayer> )



    		)
       // .init_resource::<TileEditDataResource>()
        ;
}




pub type TilePbrBundle = MaterialMeshBundle<TileMaterialExtension>;



//should spatially offset the layer at the appropriate height
#[derive(Component)]
pub struct ClayTileLayer; 

#[derive(Component)]
pub struct ClayTileMesh;


//if this exists, destroy it and rebuild the children w the data 
#[derive(Component)]
pub struct ClayTileLayerBuildData{


	pub tile_operations: Vec<ClayTileOperation>
} 






pub fn build_tile_layer_meshes(
	mut commands:Commands,
	clay_tile_layer_query: Query<
	 (Entity, & ClayTileLayer, &ClayTileLayerBuildData, &Parent )
	>, 

	 mut meshes: ResMut<Assets<Mesh>>,

    tile_root_query: Query<(&ClayTilesRoot, &ClayTilesConfig)>,
    mut tile_materials: ResMut<Assets<TileMaterialExtension>>,


	){


	for (layer_entity, _, build_data, parent ) in clay_tile_layer_query.iter(){


		commands.entity(layer_entity).remove::<ClayTileLayerBuildData>();

		
		let Some((clay_tiles_root,clay_tiles_config)) = tile_root_query.get(parent.get()).ok() else {continue};

		//let tile_diffuse_texture = clay_tiles_root.terrain_data_loaded
		let tile_diffuse_texture = clay_tiles_root.get_diffuse_texture_image().clone();
            

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



             let mesh = build_mesh_from_operations( build_data.tile_operations.as_ref() );


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

            commands.entity(layer_entity)
             .add_child(mesh_bundle);
            
           // chunk_data.chunk_state = ChunkState::FullyBuilt;

            println!("chunk fully built ");

           // commands.entity(entity).despawn();

     }

}