use geo::Polygon;
use geo::LineString;





#[derive(Clone,Debug)]
pub struct ClayTileOperation {


	pub height_layer: usize,
	pub operation_type: OperationType, 


	pub shape_type: ClayShapeType,

	pub dimensions: [[i32;2] ;2 ]  //start x,y,  size x,y 




}



impl ClayTileOperation {


	pub fn to_linestring(&self) -> LineString {

		 let [[start_x, start_y], [size_x, size_y]] = self.dimensions;
	    let exterior = LineString::from(vec![
	        (start_x as f64, start_y as f64),
	        ((start_x + size_x) as f64, start_y as f64),
	        ((start_x + size_x) as f64, (start_y + size_y) as f64),
	        (start_x as f64, (start_y + size_y) as f64),
	        (start_x as f64, start_y as f64), // Ensure the polygon is closed
	    ]);

	    exterior
	}

	pub fn to_exterior_polygon(&self)  -> Polygon {

		 Polygon::new(self.to_linestring(), vec![])
	}

}











#[derive(Clone,Debug)]
pub enum OperationType {

	Union,
	Difference,



}

#[derive(Clone,Debug)]
pub enum ClayShapeType {

	Rectangle, 
	Hexagon 
}