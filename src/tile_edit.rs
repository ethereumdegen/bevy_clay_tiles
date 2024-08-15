
use core::f32::consts::PI;
use bevy::{prelude::* };
use geo::{Polygon, LineString, BooleanOps};
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology::TriangleList;



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



       // Convert Vec<(f64, f64)> to geo::LineString
    let exterior1 = LineString::from(vec![
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
    ]);

    // Create geo::Polygon from LineString
    let poly1 = Polygon::new(exterior1, vec![]);
    let poly2 = Polygon::new(exterior2, vec![]);

    // Apply the difference operation using geo's BooleanOps
    let result_polygon = poly1.difference(&poly2);

    // Extrude the resulting 2D shape into a 3D mesh
    for polygon in result_polygon {
        let mesh = extrude_polygon_to_3d(&polygon , 0.2 );

        let material_color =  Color::srgb(0.8, 0.7, 0.6); 


        // Add the extruded mesh to the scene
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add( material_color ),
            ..Default::default()
        });
    }
}

fn extrude_polygon_to_3d(polygon: &Polygon<f64>, height:f32 ) -> Mesh {
   let mut mesh = Mesh::new( TriangleList , RenderAssetUsages::default());

    // Create vertices
    let mut vertices = vec![];
    let mut indices = vec![];

    // Iterate over the exterior of the polygon to generate vertices
    let exterior = polygon.exterior();
    for point in exterior.coords() {
        vertices.push([point.x as f32, point.y as f32, 0.0]); // Bottom vertices
        vertices.push([point.x as f32, point.y as f32, height]); // Top vertices
    }

    let vert_count = vertices.len() as u32;

    // Create the sides by connecting top and bottom vertices
    for i in (0..vert_count).step_by(2) {
        let next = (i + 2) % vert_count;
        indices.extend_from_slice(&[
            i, i + 1, next,     // First triangle
            next, i + 1, next + 1, // Second triangle
        ]);
    }

    // Fill the top and bottom faces (optional, depending on what you need)
    for i in 0..(vert_count / 2 - 2) {
        indices.push(0);
        indices.push(i * 2 + 2);
        indices.push(i * 2 + 4);

        indices.push(1);
        indices.push(i * 2 + 3);
        indices.push(i * 2 + 5);
    }

    

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices.iter().cloned().collect::<Vec<[f32; 3]>>(),
    );
    mesh.insert_indices( Indices::U32(indices) );
    mesh
}