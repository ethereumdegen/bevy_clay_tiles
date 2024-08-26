
 use geo::Coord;
 use bevy::utils::HashMap;
use geo::algorithm::centroid::Centroid;
 
use lyon::math::{Box2D, Point, point};
use lyon::path::{Path, Winding, builder::BorderRadii};
use lyon::tessellation::{FillTessellator, FillOptions, VertexBuffers};
use lyon::tessellation::geometry_builder::simple_builder;

use crate::clay_tile::ClayTileComponent;
//use crate::clay_tile_operation::OperationType;
 
use core::f32::consts::PI;
use bevy::{prelude::* };
use geo::{MultiPolygon, BooleanOps, CoordsIter, LineString, OpType, Polygon};
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology::TriangleList;


/*

First need to do the sides - they are easy... i know the top and bottom vertices so 
i just need to make the triangles that connect them (make a fn that makes 2 triangles from a quad?)

Then need to do the top and bottom.  This is trickier as I will need to tesselate the top into triangles 

*/

pub(crate) fn tile_edit_plugin(app: &mut App) {
    app

      //  .init_resource::<TileEditDataResource>()
        ;
}


 


pub struct PreMesh {
    positions: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
}


 
impl PreMesh {
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            uvs: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
        }
    }


    fn calculate_smooth_normals(&mut self) {
        let mut vertex_normals_accum: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]; self.positions.len()];

        // Step 1: Calculate face normals and accumulate them for each vertex
        for i in (0..self.indices.len()).step_by(3) {
            let idx0 = self.indices[i] as usize;
            let idx1 = self.indices[i + 1] as usize;
            let idx2 = self.indices[i + 2] as usize;

            let v0 = self.positions[idx0];
            let v1 = self.positions[idx1];
            let v2 = self.positions[idx2];

            let normal = compute_normal(v0, v1, v2);

            // Step 2: Accumulate normals for each vertex of the face
            for &idx in &[idx0, idx1, idx2] {
                vertex_normals_accum[idx][0] += normal[0];
                vertex_normals_accum[idx][1] += normal[1];
                vertex_normals_accum[idx][2] += normal[2];
            }
        }

        // Step 3: Normalize accumulated normals to average them
        for normal in vertex_normals_accum.iter_mut() {
            let len =
                f32::sqrt(normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]);
            if len > 0.0 {
                normal[0] /= len;
                normal[1] /= len;
                normal[2] /= len;
            }
        }

        // Step 4: Assign averaged normals to the mesh
        self.normals = vertex_normals_accum;
    }


    fn add_triangle(&mut self, positions: [[f32; 3]; 3], uvs: [[f32; 2]; 3]) {
        // Add vertices and indices
        for psn in &positions {
            //   println!("psn {:?}", psn);
            self.positions.push(*psn);
        }
        let start_idx = self.positions.len() as u32 - 3;
        self.indices
            .extend(&[start_idx, start_idx + 1, start_idx + 2]);

        //stubbed in for now ...
        // let normal = compute_normal(positions[0], positions[1], positions[2]);
        //self.normals.extend([normal, normal, normal]);

        self.uvs.extend(uvs);
    }

    pub fn build(self) -> Mesh {
        let mut mesh = Mesh::new(TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_indices(Indices::U32(self.indices));

       // mesh.generate_tangents().unwrap(); 
        mesh
    }





// Function to extrude a Polygon into a 3D mesh
pub fn extrude_2d_polygon_to_3d(
    polygon: &MultiPolygon , 
    origin_offset: IVec2,
    height: f64,
    bevel_factor: f64,
     additional_points_translation: Option< &HashMap<usize,IVec2> >
    ) -> Option<Self> {
    


    let mut premesh = Self::new();


     let shape_2d_vertex_count = polygon.exterior_coords_iter().count().clone();

    // Iterate over the exterior of the polygon
  //  let exterior_coords = polygon.exterior_coords_iter() ;

    let mut exterior_coords: Vec<Coord<f64>> = polygon
        .exterior_coords_iter()
        .map(|coord|  
              Coord {
                 x: coord.x + origin_offset.x as f64,
                 y: coord.y + origin_offset.y as f64,
             }

        )
        .collect();





     //modify by additional translate

        if let Some(translations) = additional_points_translation {
            for (point_index, translation) in translations.iter() {
                if let Some(coord) = exterior_coords.get_mut(*point_index) {
                    coord.x += translation.x as f64;
                    coord.y += translation.y as f64;
                }
            }
        }
 

    let mut bottom_vertices = Vec::new();
    let mut top_vertices = Vec::new();

    // Create bottom and top vertices
    for coord in exterior_coords {
        // Bottom vertices (y = 0)
        bottom_vertices.push( [coord.x as f32, 0.0, coord.y as f32] );
        // Top vertices (y = height)
        top_vertices.push( [coord.x as f32, height as f32, coord.y as f32]);
    }




   




     // Compute the centroid of the top vertices and scale them in 
    if let Some(centroid) = polygon.centroid() {
        let centroid = [centroid.x() as f32, height as f32, centroid.y() as f32];

        // Scale top vertices towards the centroid based on the bevel factor
        for vertex in &mut top_vertices {
            vertex[0] = vertex[0] + (centroid[0] - vertex[0]) * bevel_factor as f32;
            vertex[2] = vertex[2] + (centroid[2] - vertex[2]) * bevel_factor as f32;
        }
    }



//fix the uv unwrapping here.. 

        let uv_coords_height = height.clone() as f32;

       // let uv_coords_height_scaled = 1.0 / ( uv_coords_height / (1.0 - bevel_factor as f32) );
       let uv_coords_height_scaled = 1.0 ;
        let uv_coords_width_scaled = 1.0  ;

    // Add triangles for the sides --uv are broken ? 
        for i in 0..shape_2d_vertex_count {




            let next_index = (i + 1) % shape_2d_vertex_count;
            let positions  = [
                
                top_vertices[i],
                bottom_vertices[i],
                top_vertices[next_index],
            ]; 
            
            let horizontal_distance = Vec2::new( bottom_vertices[i][0], bottom_vertices[i][2] ).distance(  Vec2::new( bottom_vertices[next_index][0], bottom_vertices[next_index][2] ) ); 
                

            let vertical_distance = (bottom_vertices[i][1] as f32 - top_vertices[i][1]as f32).abs()  ; 
            

            let horizontal_distance_uv_factor =  horizontal_distance;
            let vertical_distance_uv_factor =  vertical_distance;

              //  info!("uv horizontal_distance_uv_factor {:?}", horizontal_distance_uv_factor );


                //   info!("uv vertical_distance_uv_factor {:?}", vertical_distance_uv_factor );
          /*  let uv_coords = [
               
                [uv_coords_width_scaled * horizontal_distance_uv_factor, 0.0],
                 [0.0, 0.0],
                [uv_coords_width_scaled * horizontal_distance_uv_factor, uv_coords_height_scaled * vertical_distance_uv_factor],
            ];*/
            let uv_coords = [
                [0.0, uv_coords_height_scaled * vertical_distance_uv_factor],
                [0.0, 0.0],
                [uv_coords_width_scaled * horizontal_distance_uv_factor, uv_coords_height_scaled * vertical_distance_uv_factor],
            ];
            premesh.add_triangle(positions , uv_coords);

            let positions = [
                
                top_vertices[next_index],
                bottom_vertices[i],
                bottom_vertices[next_index],
            ];
           /* let uv_coords = [
               
                [uv_coords_width_scaled * horizontal_distance_uv_factor, uv_coords_height_scaled * vertical_distance_uv_factor],
                 [0.0, 0.0],
                [0.0, uv_coords_height_scaled * vertical_distance_uv_factor],
            ];*/
            let uv_coords = [
                [uv_coords_width_scaled * horizontal_distance_uv_factor,  uv_coords_height_scaled * vertical_distance_uv_factor],
                [0.0, 0.0],
                [uv_coords_width_scaled * horizontal_distance_uv_factor, 0.0],
            ];
            premesh.add_triangle(positions , uv_coords);
        }

    // premesh.add_triangle([left_front, right_back, left_back], [uv_lf, uv_rb, uv_lb]);





    // use lyon to tesselate the shape2d input and then use that tesselation to add triangles for the top and then the bottom 

     // Tessellation for the top and bottom faces
        let mut tessellator = FillTessellator::new();
        let mut top_side_buffers: VertexBuffers<Point, u16> = VertexBuffers::new();
        let mut bottom_side_buffers: VertexBuffers<Point, u16> = VertexBuffers::new();

        // ---- build paths 
            let mut top_side_builder = Path::builder();
 
             let mut has_placed_first_point = false;

            // Building the path from the adjusted top vertices
            for vertex in &top_vertices {
                if !has_placed_first_point {
                    top_side_builder.begin(point(vertex[0], vertex[2]));
                    has_placed_first_point = true;
                } else {
                    top_side_builder.line_to(point(vertex[0], vertex[2]));
                }
            }
            top_side_builder.close();



            let mut bottom_side_builder = Path::builder();
 
             let mut has_placed_first_point = false;

            // Building the path from the adjusted top vertices
            for vertex in &bottom_vertices {
                if !has_placed_first_point {
                    bottom_side_builder.begin(point(vertex[0], vertex[2]));
                    has_placed_first_point = true;
                } else {
                    bottom_side_builder.line_to(point(vertex[0], vertex[2]));
                }
            }
            bottom_side_builder.close();
            // -----


            //-- tesselate 
            let top_side_path = top_side_builder.build();
            // Tesselate the path
            tessellator.tessellate_path(
                &top_side_path,
                &FillOptions::tolerance(0.01),
                &mut simple_builder(&mut top_side_buffers),
            ).unwrap();

             let bottom_side_path = bottom_side_builder.build();
            // Tesselate the path
            tessellator.tessellate_path(
                &bottom_side_path,
                &FillOptions::tolerance(0.01),
                &mut simple_builder(&mut bottom_side_buffers),
            ).unwrap();

            //---
         

            // Create top and bottom faces from tessellation
        for triangle in top_side_buffers.indices.chunks(3) {
           // let mut bottom_triangle = [Default::default(); 3];
            let mut top_triangle = [Default::default(); 3];
            let mut uvs = [Default::default(); 3];

            for (i, &index) in triangle.iter().enumerate() {
                let vertex = top_side_buffers.vertices[index as usize];
               // bottom_triangle[i] = [vertex.x, 0.0, vertex.y];  // Correct as is
                top_triangle[i] = [vertex.x, height as f32, vertex.y];  // Correct as is
                uvs[i] = [vertex.x, vertex.y]; // Basic UV mapping; adjust as necessary
            }

             // info!("add top tri {:?}",top_triangle);
            premesh.add_triangle(top_triangle , uvs);  // Adding top face
           // premesh.add_triangle(bottom_triangle , uvs);  // Adding bottom face with corrected winding order
        }

         for triangle in bottom_side_buffers.indices.chunks(3) {
            let mut bottom_triangle = [Default::default(); 3];
          //  let mut top_triangle = [Default::default(); 3];
            let mut uvs = [Default::default(); 3];

            for (i, &index) in triangle.iter().enumerate() {
                let vertex = bottom_side_buffers.vertices[index as usize];

                let j = 2 - i; //flip normals 
                bottom_triangle[j] = [vertex.x, 0.0, vertex.y];  // Correct as is
               // top_triangle[i] = [vertex.x, height as f32, vertex.y];  // Correct as is
                uvs[j] = [vertex.x, vertex.y]; // Basic UV mapping; adjust as necessary
            }

             // info!("add top tri {:?}",top_triangle);
          //  premesh.add_triangle(top_triangle , uvs);  // Adding top face
            premesh.add_triangle(bottom_triangle , uvs);  // Adding bottom face with corrected winding order
        }


  

      premesh.calculate_smooth_normals();

       Some( premesh )


}
 
    
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




fn compute_normal(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
    let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

    // Cross product
    let normal = [
        edge1[1] * edge2[2] - edge1[2] * edge2[1],
        edge1[2] * edge2[0] - edge1[0] * edge2[2],
        edge1[0] * edge2[1] - edge1[1] * edge2[0],
    ];

    // normal

    [normal[0], normal[1], normal[2]] //is this busted ?
}
 