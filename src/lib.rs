use bevy::prelude::*;

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Grass ground
    commands.spawn((
        Mesh3d(meshes.add(
            Plane3d::default().mesh().size(80.0, 80.0)
        )),
        MeshMaterial3d(materials.add(
            Color::srgb(0.15, 0.4, 0.12)
        )),
    ));

    // Player
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(
            Color::srgb(0.1, 0.3, 0.9)
        )),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // Trees
    let trunk = meshes.add(Cylinder::new(0.2, 1.5));
    let leaves = meshes.add(Cone::new(1.0, 2.5));

    let brown = materials.add(Color::srgb(0.3, 0.15, 0.06));
    let green = materials.add(Color::srgb(0.05, 0.3, 0.08));

    for x in [-12.0, -6.0, 6.0, 12.0] {
        for z in [-12.0, -6.0, 6.0, 12.0] {
            commands.spawn((
                Mesh3d(trunk.clone()),
                MeshMaterial3d(brown.clone()),
                Transform::from_xyz(x, 0.75, z),
            ));

            commands.spawn((
                Mesh3d(leaves.clone()),
                MeshMaterial3d(green.clone()),
                Transform::from_xyz(x, 2.5, z),
            ));
        }
    }

    // Sun
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(
            Quat::from_rotation_x(-1.0)
        ),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7.0, 13.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
