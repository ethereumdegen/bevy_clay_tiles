
//see bindings in terrain_material.rs 
 
 //https://github.com/nicopap/bevy_mod_paramap/blob/main/src/parallax_map.wgsl



 #import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
      mesh_view_bindings::view,

      pbr_bindings,
    
    pbr_fragment::pbr_input_from_standard_material,
      pbr_functions::{alpha_discard,calculate_tbn_mikktspace,apply_pbr_lighting, main_pass_post_lighting_processing,
      prepare_world_normal,
      apply_normal_mapping,
      calculate_view

      },
    // we can optionally modify the lit color before post-processing is applied
    pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT,STANDARD_MATERIAL_FLAGS_UNLIT_BIT},
}

#import bevy_core_pipeline::tonemapping::tone_mapping
  
struct StandardMaterial {
    time: f32,
    base_color: vec4<f32>,
    emissive: vec4<f32>,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
    alpha_cutoff: f32,
};

/*
struct ChunkMaterialUniforms {
    color_texture_expansion_factor: f32 ,
    chunk_uv: vec4<f32>,  //start_x, start_y, end_x, end_y   -- used to subselect a region from the splat texture 
    
};


struct ToolPreviewUniforms { 
    tool_coordinates: vec2<f32>,
    tool_radius: f32,
    tool_color: vec3<f32>    
};
*/
//https://github.com/DGriffin91/bevy_mod_standard_material/blob/main/assets/shaders/pbr.wgsl


@group(1) @binding(1)
var base_color_texture1: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler1: sampler;
 

@group(1) @binding(3)
var emissive_texture: texture_2d<f32>;
@group(1) @binding(4)
var emissive_sampler: sampler;

@group(1) @binding(5)
var metallic_roughness_texture: texture_2d<f32>;
@group(1) @binding(6)
var metallic_roughness_sampler: sampler;

@group(1) @binding(7)
var occlusion_texture: texture_2d<f32>;
@group(1) @binding(8)
var occlusion_sampler: sampler;


@group(2) @binding(20)
var<uniform>  color_texture_expansion_factor:   f32;

@group(2) @binding(21)
var<uniform>  diffuse_color_tint:  vec4<f32>;

@group(2) @binding(22)
var<uniform>  tile_texture_index: u32; 

//@group(2) @binding(21)
//var<uniform> tool_preview_uniforms: ToolPreviewUniforms;

@group(2) @binding(24)
var base_color_texture: texture_2d_array<f32>;
@group(2) @binding(25)
var base_color_sampler: sampler;

@group(2) @binding(26)
var normal_texture: texture_2d_array<f32>;
@group(2) @binding(27)
var normal_sampler: sampler;



 
  
 

//should consider adding vertex painting to this .. need another binding of course.. performs a color shift 

@fragment
fn fragment(
    mesh: VertexOutput,
    
     
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    
    
    let mesh_uv =  color_texture_expansion_factor * mesh.uv;
     
    // let tile_texture_index = 0; // do uv coord stuff ?
 
    let color_from_diffuse_texture = textureSample(base_color_texture, base_color_sampler, mesh_uv, tile_texture_index);
     

    let normal_from_texture = textureSample(normal_texture, normal_sampler, mesh_uv, tile_texture_index); 
    
 
    

    let blended_color = color_from_diffuse_texture  * diffuse_color_tint ;
    var blended_normal = normal_from_texture  ;
     blended_normal =  normalize(blended_normal); // FOR NOW  // normalize(blended_normal); 
                    
   let blended_normal_vec3 = vec3<f32>( blended_normal.r, blended_normal.g, blended_normal.b );         
   
   
    var pbr_input = pbr_input_from_standard_material(mesh, is_front);
        
    pbr_input.material.base_color =  blended_color;
  
      let double_sided = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;
 
     
    pbr_input.world_position = mesh.world_position ;
    pbr_input.world_normal =  prepare_world_normal(
        mesh.world_normal,
        double_sided,
        is_front,
    );

// https://github.com/bevyengine/bevy/blob/main/assets/shaders/array_texture.wgsl 
    
   /* let tangent = normalize( blended_normal_vec3 );

    //we mix the normal with our sample so shadows are affected by the normal map ! 
    let normal_mixed = mix( normalize( mesh.world_normal ) , normalize( tangent ) , 0.7 );

     let TBN = calculate_tbn_mikktspace(normalize(normal_mixed ), vec4(tangent,1.0 )  ) ;  //for anistropy ??

 

    let Nt = textureSampleBias(
        pbr_bindings::normal_map_texture,
         pbr_bindings::normal_map_sampler, mesh.uv, view.mip_bias).rgb;

    */
 
    pbr_input.N =   blended_normal_vec3;
 

    pbr_input.V =  calculate_view(mesh.world_position, pbr_input.is_orthographic);


    var pbr_out: FragmentOutput;
 
    
    // apply lighting
    pbr_out.color = apply_pbr_lighting(pbr_input);
    
    pbr_out.color = main_pass_post_lighting_processing(pbr_input, pbr_out.color);

    pbr_out.color=  tone_mapping(pbr_out.color, view.color_grading);

    // -----

   // let shadowFactor = calculate_shadow_factor(frag_lightSpacePos);


   
   // let vertex_world_psn = mesh.world_position.xz; // Assuming the vertex position is in world space

   // let tool_coordinates = tool_preview_uniforms.tool_coordinates;
   // let tool_radius = tool_preview_uniforms.tool_radius;
   // let color_from_tool = tool_preview_uniforms.tool_color;

   // let distance = length(vertex_world_psn - tool_coordinates);

  //  let within_tool_radius = f32(distance <= tool_radius);


    //need to fix lighting !!! 
    
    var final_color = vec4( pbr_out.color.rgb, 1.0);
    

     let unlit = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) != 0u;
    
    if unlit {
         final_color = vec4( blended_color.rgb, 1.0);

    }


     
    
    return final_color;
    
}
 

 