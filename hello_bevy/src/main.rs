//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_rotation)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
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
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_rotation(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    const ROTATION_SPEED: f32 = 1.0;

    for mut transform in query.iter_mut() {
        if keys.pressed(KeyCode::ArrowLeft) {
            transform.rotate_y(ROTATION_SPEED * -0.02);
        }
        if keys.pressed(KeyCode::ArrowRight) {
            transform.rotate_y(ROTATION_SPEED * 0.02);
        }
        if keys.pressed(KeyCode::ArrowUp) {
            transform.rotate_x(ROTATION_SPEED * -0.02);
        }
        if keys.pressed(KeyCode::ArrowDown) {
            transform.rotate_x(ROTATION_SPEED * 0.02);
        }
    }
}

// //! Shows how to render simple primitive shapes with a single color.
// //!
// //! You can toggle wireframes with the space bar except on wasm. Wasm does not support
// //! `POLYGON_MODE_LINE` on the gpu.

// #[cfg(not(target_arch = "wasm32"))]
// use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
// use bevy::{
//     prelude::*,
//     sprite::{MaterialMesh2dBundle, Mesh2dHandle},
// };

// fn main() {
//     let mut app = App::new();
//     app.add_plugins((
//         DefaultPlugins,
//         #[cfg(not(target_arch = "wasm32"))]
//         Wireframe2dPlugin,
//     ))
//     .add_systems(Startup, setup);
//     #[cfg(not(target_arch = "wasm32"))]
//     app.add_systems(Update, toggle_wireframe);
//     app.run();
// }

// const X_EXTENT: f32 = 900.;

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(Camera2dBundle::default());

//     let shapes = [
//         Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
//         Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
//         Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
//         Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
//         Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
//         Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
//         Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
//         Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
//         Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
//         Mesh2dHandle(meshes.add(Triangle2d::new(
//             Vec2::Y * 50.0,
//             Vec2::new(-50.0, -50.0),
//             Vec2::new(50.0, -50.0),
//         ))),
//     ];
//     let num_shapes = shapes.len();

//     for (i, shape) in shapes.into_iter().enumerate() {
//         // Distribute colors evenly across the rainbow.
//         let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

//         commands.spawn(MaterialMesh2dBundle {
//             mesh: shape,
//             material: materials.add(color),
//             transform: Transform::from_xyz(
//                 // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
//                 -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
//                 0.0,
//                 0.0,
//             ),
//             ..default()
//         });
//     }

//     #[cfg(not(target_arch = "wasm32"))]
//     commands.spawn(
//         TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
//             .with_style(Style {
//                 position_type: PositionType::Absolute,
//                 top: Val::Px(12.0),
//                 left: Val::Px(12.0),
//                 ..default()
//             }),
//     );
// }

// #[cfg(not(target_arch = "wasm32"))]
// fn toggle_wireframe(
//     mut wireframe_config: ResMut<Wireframe2dConfig>,
//     keyboard: Res<ButtonInput<KeyCode>>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         wireframe_config.global = !wireframe_config.global;
//     }
// }
