 




 use geo::{CoordsIter, MultiPolygon};

use bevy::prelude::*; 


pub(crate) fn tile_edit_plugin(app: &mut App) {
    app

      //  .init_resource::<TileEditDataResource>()
        ;
}

pub fn extrude_polygon_to_3d(
    polygon: &MultiPolygon,
    height: f64,
) -> Option<(Vec<[f64; 3]>, Vec<[usize; 3]>, Vec<[f32; 2]>, Vec<[f64; 3]>)> {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();

    let vertex_count = polygon.exterior_coords_iter().count().clone();

    // Iterate over the exterior of the polygon
    let exterior_coords = polygon.exterior_coords_iter();

    // Create bottom and top vertices and normals
    for coord in exterior_coords {
        // Bottom vertices (y = 0)
        vertices.push([coord.x, 0.0, coord.y]);
        normals.push([0.0, -1.0, 0.0]); // Bottom normals (pointing down)

        // Top vertices (y = height)
        vertices.push([coord.x, height, coord.y]);
        normals.push([0.0, 1.0, 0.0]); // Top normals (pointing up)
    }

    // Connect vertices to form triangles for the sides
    for i in 0..(vertex_count - 1) {
        let bottom1 = 2 * i;
        let bottom2 = 2 * (i + 1);
        let top1 = bottom1 + 1;
        let top2 = bottom2 + 1;

        // Side triangles
        indices.push([top1, bottom1, bottom2]);
        indices.push([top2, top1, bottom2]);

        // Calculate normal for the side face
        let edge = [
            vertices[bottom2][0] - vertices[bottom1][0],
            vertices[bottom2][1] - vertices[bottom1][1],
            vertices[bottom2][2] - vertices[bottom1][2],
        ];
        let normal = cross_product(edge, [0.0, 1.0, 0.0]); // Cross with Y-axis to get normal

        normals[bottom1] = normal;
        normals[bottom2] = normal;
        normals[top1] = normal;
        normals[top2] = normal;
    }

    // Closing the top and bottom faces
    for i in 1..(vertex_count - 1) {
        // Bottom
        indices.push([0, 2 * i, 2 * (i + 1)]);
        // Top
        indices.push([1, 2 * i + 1, 2 * (i + 1) + 1]);
    }

    // Generate UV coordinates
    let uvs: Vec<[f32; 2]> = generate_uvs(&vertices);

    Some((vertices, indices, uvs, normals))
}

// Utility function to compute the cross product of two vectors
fn cross_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
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
