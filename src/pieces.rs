use bevy::prelude::*;

pub fn spawn_king(
    mut commands: Commands,
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