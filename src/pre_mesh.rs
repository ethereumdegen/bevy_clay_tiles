
use crate::clay_tile::ClayTileComponent;
//use crate::clay_tile_operation::OperationType;
 


use core::f32::consts::PI;
use bevy::{prelude::* };
use geo::{MultiPolygon, BooleanOps, CoordsIter, LineString, OpType, Polygon};
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology::TriangleList;



pub(crate) fn tile_edit_plugin(app: &mut App) {
    app

      //  .init_resource::<TileEditDataResource>()
        ;
}



/*
#[derive(Resource,Default)]
pub struct TileEditDataResource {

    tile_operations: Vec<ClayTileOperation>

}*/


/*

	
	let operations = vec![
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
    ];


	*/


/*
pub fn build_mesh_from_operations(

	operations: &Vec<ClayTileOperation>

	) -> Option<Mesh> {

	

	let result_polygon = build_combined_polygon(operations);
    
  //  result_polygon.exterior_coords_iter()

  let Some(result_polygon) = result_polygon else {
  	return None
  } ;

   let (vertices, indices, uvs) = extrude_polygon_to_3d( &result_polygon , 0.2  );

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
 

fn build_combined_polygon(operations: &Vec<ClayTileOperation>) -> Option<MultiPolygon> {
    if operations.is_empty() {
        return None;
    }

    let first_polygon = &operations[0];

    let mut result_polygon :MultiPolygon = first_polygon.to_exterior_polygon() .into();

    for next_operation in &operations[1..] {
        let poly:MultiPolygon = next_operation.to_exterior_polygon().into();

        result_polygon = match next_operation.operation_type {
            OperationType::Union => result_polygon.union(&poly),
            OperationType::Difference => result_polygon.difference(&poly),
            // Add more operations as needed
        };
    }
 

  //  let result_polygon = poly1.boolean_op(&poly2,  OpType::Union);


    Some( result_polygon )
}*/


// Function to extrude a Polygon into a 3D mesh
pub fn extrude_polygon_to_3d(polygon: &MultiPolygon , height: f64) -> Option< (Vec<[f64; 3]>, Vec<[usize; 3]>,Vec<[f32; 2]>) > {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let vertex_count = polygon.exterior_coords_iter().count().clone();

    // Iterate over the exterior of the polygon
    let exterior_coords = polygon.exterior_coords_iter() ;

    // Create bottom and top vertices
    for coord in exterior_coords {
        // Bottom vertices (y = 0)
        vertices.push( [coord.x, 0.0, coord.y] );
        // Top vertices (y = height)
        vertices.push( [coord.x, height, coord.y ]);
    }

    

    // Connect vertices to form triangles for the sides
    for i in 0..(vertex_count - 1) {
        let bottom1 = 2 * i;
        let bottom2 = 2 * (i + 1);
        let top1 = bottom1 + 1;
        let top2 = bottom2 + 1;

        // Side triangles
        indices.push([top1,bottom1,  bottom2]);
        indices.push([top2, top1,  bottom2]);
    }

    // Closing the top and bottom (optional depending on the desired effect)
    for i in 1..(vertex_count - 1) {
        // Bottom
        indices.push([ 0, 2 * i, 2 * (i + 1)]);
        // Top
        indices.push([1,  2 * i + 1,  2 * (i + 1) + 1]);
    }


     // Generate UV coordinates
    let uvs: Vec<[f32; 2]> = generate_uvs(&vertices);


   Some(  (vertices, indices , uvs) )
}


pub fn generate_uvs(vertices: &Vec<[f64; 3]>) -> Vec<[f32; 2]> {
    vertices.iter().map(|&[x, _, z]| [x as f32, z as f32]).collect()
}

pub fn point3_to_array_f32(point: &[f64; 3]) -> [f32; 3] {
    [point[0] as f32, point[1] as f32, point[2] as f32]
}

// Function to flatten indices
pub fn flatten_indices(indices: &Vec<[usize; 3]>) -> Vec<u32> {
    indices.iter().flat_map(|&[a, b, c]| vec![a as u32, b as u32, c as u32]).collect()
}
