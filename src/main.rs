use bevy::prelude::*;

fn main() {
    App::new()
    // set antialiasing to 4
    .insert_resource(Msaa { samples: 4})
    // set WindowDescriptor Resource to change title and size
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor { 
            width: 1600.0,
            height: 1600.0,
            title: "Chessboard".to_string(),
            ..default()   
        },
        ..default()
    }))
    .run();
}
