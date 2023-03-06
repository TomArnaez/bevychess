use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 1600.;
const SCREEN_HEIGHT: f32 = 1600.;

fn main() {
    App::new()
    // set antialiasing to 4
    .insert_resource(Msaa { samples: 4})
    // set WindowDescriptor Resource to change title and size
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor { 
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Chessboard".to_string(),
            ..default()   
        },
        ..default()
    }))
    .add_startup_system(setup)
    .add_startup_system(create_board)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..default()
    });
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.}));
    let white_material = materials.add(Color::WHITE.into());
    let black_material = materials.add(Color::BLACK.into());

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                // Change material according to position to get alternating pattern
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..default()
            });
        }
    }
}