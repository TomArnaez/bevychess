use bevy::prelude::*;

pub fn spawn_rook( 
    mut commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
    // Spawn parent entity
    .spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(-0.1, 0.0, 1.8),
                scale: Vec3::new(0.2, 0.2, 0.2),
                ..default()
            },
            ..default()
        });
    });
}

pub fn spawn_knight(
    mut commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh_1,
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(-0.2, 0., 0.9),
                    scale: Vec3::new(0.2, 0.2, 0.2),
                    ..default()
                },
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_2,
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(-0.2, 0., 0.9),
                    scale: Vec3::new(0.2, 0.2, 0.2),
                    ..default()
                },
                ..default()
            });
        });
}

pub fn spawn_bishop( 
    mut commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
    // Spawn parent entity
    .spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(-0.1, 0.0, 0),
                scale: Vec3::new(0.2, 0.2, 0.2),
                ..default()
            },
            ..default()
        });
    });
}

pub fn spawn_pawn( 
    mut commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
    // Spawn parent entity
    .spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(-0.2, 0.0, 2.6),
                scale: Vec3::new(0.2, 0.2, 0.2),
                ..default()
            },
            ..default()
        });
    });
}

pub fn spawn_king(
    mut commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(-0.2, 0.0, -1.9),
                    scale: Vec3::new(0.2, 0.2, 0.2),
                    ..default()
                },
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_cross,
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(-0.2, 0.0, -1.9),
                    scale: Vec3::new(0.2, 0.2, 0.2),
                    ..default()
                },
                ..default()
            });
        });
}

pub fn spawn_queen( 
    mut commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
    // Spawn parent entity
    .spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(-0.2, 0.0, -0.95),
                scale: Vec3::new(0.2, 0.2, 0.2),
                ..default()
            },
            ..default()
        });
    });
}