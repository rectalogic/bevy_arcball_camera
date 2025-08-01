use bevy::{color::palettes, prelude::*, window::WindowResolution};
use bevy_arcball_camera::{ArcballCamera, ArcballCameraPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(500.0, 500.0),
                ..default()
            }),
            ..default()
        }),
        ArcballCameraPlugin,
    ))
    .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(palettes::css::RED))),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(palettes::css::GREEN))),
        Transform::from_xyz(-1.5, 1.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 2.0))),
        MeshMaterial3d(materials.add(Color::from(palettes::css::MEDIUM_BLUE))),
        Transform::from_xyz(0.0, -1.0, 1.5),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 3.0, 2.0)));
    commands.spawn((Camera3d::default(), ArcballCamera::new(5.0)));
}
