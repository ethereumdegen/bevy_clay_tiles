
 ## Bevy Clay Tiles

 Paint tiles and walls within a constrained+layered 2D grid system and they are extruded to 3D meshes for 3D games.  This is meant to be useful for blocking out levels.



(Inspired by DreamerTalin)


### Installing
```
cargo add bevy_clay_tiles
```


![clay_tiles](https://github.com/user-attachments/assets/2436b6bd-fff8-4edb-982b-69c5a03a1258)
 

  ![image](https://github.com/user-attachments/assets/e0f18271-ccba-479a-a7e5-9c5a7f68b902)



### Run example 

```
cargo run --example basic
```

 
### Config 

In the tile_types config, for each 'tile type' you can set:

1. the texture index used for diffuse texture
2. texture UV expansion factor (stretch material)
3. color tint of the diffuse texture


### Editing 

When creating the clay tiles, you can use either a Rectangle paint mode or a Polygon paint mode, similar to the Rectangle or Polygon select tools in typical photo editor softwares.  

The edit mode allows you to control: 

1. the Y height offset of created tiles 
2. the height scale that new tiles will be extruded up to 
3. the tile type index (see the Config section and tile_types.ron) 
4. the default parent entity for the tile meshes when created

 
 


### Tile Materials 

When clay tiles blocks spawn, they will not have a material on them, only a component ClayTileMaterial { material_name: String }.  Therefore it is your responsibility to replace the component with your own material handle.   The example use the BevyMaterialTool to help with this but you can just insert a Handle<StandardMaterial> if you wish.

  ```

fn add_material_handles(
    mut commands:Commands, 
    block_query: Query<(Entity, &ClayTileMaterial), Added<ClayTileMaterial>>
){

    for (tile_entity, tile_material_comp) in block_query.iter(){

        let material_name = &tile_material_comp.material_name;  

        commands.get_entity(tile_entity).map( |mut cmd| { 


            cmd.remove::<ClayTileMaterial>( ); 

            cmd.insert( MaterialOverrideComponent {
                material_override:  material_name.clone()
            }  ); 


        } );

    }

}
  ```



### Do you like this crate?  Support the creator on Patreon
https://www.patreon.com/infernaltoast 
 
 [![creatinggames-preview](https://github.com/user-attachments/assets/7e7904c1-5f2b-47b6-84dd-5626cb7baca0)](https://www.patreon.com/infernaltoast)
