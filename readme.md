
 ## Bevy Clay Tiles

 Paint tiles and walls within a constrained+layered 2D grid system and they are extruded to 3D meshes for 3D games.  This is meant to be useful for blocking out levels.



(Inspired by DreamerTalin)


### Installing
```
cargo add bevy_clay_tiles
```
![image](https://github.com/user-attachments/assets/e8d28fed-02b0-47e1-971e-8198d6ac5dbf)

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

 
 
