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
    .add_startup_system(create_pieces)
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

mod pieces;
use pieces::*;

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle:Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle:Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle:Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle:Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle:Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle:Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    // Add some materailsl
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    spawn_rook(
        &mut commands,
        white_material.clone(),
        rook_handle.clone(),
        Vec3::new(0., 0., 0.),
    );

    spawn_knight(
        &mut commands,
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(0., 0., 1.),
    );

    spawn_king(
        &mut commands,
        white_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        Vec3::new(0., 0., 4.)
    );
}