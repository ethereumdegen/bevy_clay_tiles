
use crate::clay_tile::ClayTileComponent;
use crate::clay_tile_operation::OperationType;
use crate::clay_tile_operation::ClayShapeType;
use crate::clay_tile_operation::ClayTileOperation;
use core::f32::consts::PI;
use bevy::{prelude::* };
use geo::{MultiPolygon, BooleanOps, CoordsIter, LineString, OpType, Polygon};
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology::TriangleList;



pub(crate) fn tile_edit_plugin(app: &mut App) {
    app

        .init_resource::<TileEditDataResource>()
        ;
}




#[derive(Resource,Default)]
pub struct TileEditDataResource {

    tile_operations: Vec<ClayTileOperation>

}


pub fn draw_grid_gizmo(
  mut gizmos: Gizmos,

   ){ 


	   gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(100),
        Vec2::splat(1.),
        // Light gray
        LinearRgba::gray(0.95),
    );


     // UVec2::new(10, 10),
      //  Vec2::splat(2.),


}



pub fn build_tile_layer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {	


	//these are the brushes !! 

	// always ignore the FIRST brushes operation type. 

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

       // Convert Vec<(f64, f64)> to geo::LineString
   /* let exterior1 = LineString::from(vec![
        (-0.5, -0.5),
        (0.5, -0.5),
        (0.5, 0.5),
        (-0.5, 0.5),
        (-0.5, -0.5), // Ensure the polygon is closed
    ]);

    let exterior2 = LineString::from(vec![
        (-0.2, -0.2),
        (0.2, -0.2),
        (0.2, 0.2),
        (-0.2, 0.2),
        (-0.2, -0.2), // Ensure the polygon is closed
    ]); */

    // Create geo::Polygon from LineString
    

    // Apply the difference operation using geo's BooleanOps
    //let result_polygon = poly1.boolean_op(&poly2,  OpType::Union);

    let result_polygon = build_combined_polygon(operations);
    
  //  result_polygon.exterior_coords_iter()

  let Some(result_polygon) = result_polygon else {return} ;

   let (vertices, indices) = extrude_polygon_to_3d( &result_polygon , 0.2  );

     // Convert vertices to the expected format for Bevy
    let vertex_positions: Vec<[f32; 3]> = vertices.iter().map(point3_to_array_f32).collect();

    // Flatten indices for Bevy
    let flattened_indices = flatten_indices(&indices);

   let mut mesh = Mesh::new(TriangleList, RenderAssetUsages::default());

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertex_positions,
    );
    mesh.insert_indices(Indices::U32(flattened_indices));

   let material_color =  Color::srgb(0.8, 0.7, 0.6); 

   commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add( material_color ),
            ..Default::default()
        }).insert(ClayTileComponent);


    // Extrude the resulting 2D shape into a 3D mesh
    /*for polygon in result_polygon {
        let mesh = extrude_polygon_to_3d(&polygon , 0.2 );

        let material_color =  Color::srgb(0.8, 0.7, 0.6); 


        // Add the extruded mesh to the scene
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add( material_color ),
            ..Default::default()
        });
    }*/
}


fn build_combined_polygon(operations: Vec<ClayTileOperation>) -> Option<MultiPolygon> {
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

    /*let mut result_polygon = clay_tile_operation_to_polygon(&operations[0]);

    for operation in &operations[1..] {
        let poly = clay_tile_operation_to_polygon(operation);

        result_polygon = match operation.operation_type {
            OperationType::Union => result_polygon.union(&poly),
            OperationType::Difference => result_polygon.difference(&poly),
            // Add more operations as needed
        };
    }*/


  //  let result_polygon = poly1.boolean_op(&poly2,  OpType::Union);


    Some( result_polygon )
}


// Function to extrude a Polygon into a 3D mesh
fn extrude_polygon_to_3d(polygon: &MultiPolygon , height: f64) -> (Vec<[f64; 3]>, Vec<[usize; 3]>) {
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
        indices.push([bottom1, top1, bottom2]);
        indices.push([top1, top2, bottom2]);
    }

    // Closing the top and bottom (optional depending on the desired effect)
    for i in 1..(vertex_count - 1) {
        // Bottom
        indices.push([ 2 * i,0, 2 * (i + 1)]);
        // Top
        indices.push([ 2 * i + 1,1, 2 * (i + 1) + 1]);
    }

    (vertices, indices)
}

fn point3_to_array_f32(point: &[f64; 3]) -> [f32; 3] {
    [point[0] as f32, point[1] as f32, point[2] as f32]
}

// Function to flatten indices
fn flatten_indices(indices: &Vec<[usize; 3]>) -> Vec<u32> {
    indices.iter().flat_map(|&[a, b, c]| vec![a as u32, b as u32, c as u32]).collect()
}
