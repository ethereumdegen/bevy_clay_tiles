

use bevy::prelude::*;
/*

in the mesh-build step, 
you take all operations: 
  WITH the same tiletype AND  IN the same layer 

  Then you apply them together using GEO 

  Then they are extruded into a mesh .
*/

//this is a single shape, a single 'operation'
pub struct ClayTileBrush;

#[derive(Component)]
pub struct ClayTileComponent;