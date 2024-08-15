use crate::tile_edit::draw_grid_gizmo;
use crate::tile_edit::build_tile_layer;
use bevy::prelude::*;
 

 use bevy_clay_tiles::BevyClayTilesPlugin;
 use bevy_clay_tiles::tile_edit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyClayTilesPlugin {})
        //.add_startup_system(setup)
        .add_systems(Startup, (setup, build_tile_layer) )
        .add_systems(Update, (rotate_camera, draw_grid_gizmo) )
        .run();
}

fn setup( 
    mut commands: Commands,

     mut config_store: ResMut<GizmoConfigStore>,
     ) {
   
     commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 7.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

      // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

        commands.insert_resource(AmbientLight {
        color: Color::srgb(1.0,1.0,1.0),
        brightness: 700.0,
    });


         let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
          config.line_width = 2.0;
           config.enabled = true;
            config.line_style = GizmoLineStyle::Solid;

     /* commands.spawn(
        TextBundle::from_section(
            "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
            Press 'P' to toggle perspective for line gizmos\n\
            Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
            Hold 'Up' or 'Down' to change the line width of round gizmos\n\
            Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos\n\
            Press 'A' to show all AABB boxes\n\
            Press 'U' or 'I' to cycle through line styles for straight or round gizmos\n\
            Press 'J' or 'K' to cycle through line joins for straight or round gizmos",
            TextStyle::default(),
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    ); */


}

 

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 20.));
}


