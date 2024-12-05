use bevy::prelude::*;

const SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, add_characters))
        .add_systems(Update, (move_player, keep_above_ground))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn add_characters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player1 = (
        Object,
        PlayableCharacter,
        CharacterFaction::BlueFaction,
        Name("Player 1".to_string()),
        Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
        MeshMaterial2d(materials.add(Color::rgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    );
    let enemy1 = (
        Object,
        NonPlayableCharacter,
        CharacterFaction::RedFaction,
        Name("Enemy 1".to_string()),
        Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
        MeshMaterial2d(materials.add(Color::rgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    );

    commands.spawn(player1);
    commands.spawn(enemy1);
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>, // Get input resource to check for key presses
    mut query: Query<&mut Transform, With<PlayableCharacter>>,
) {
    for mut transform in &mut query {
        // Move the player based on key presses
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += SPEED * time.delta().as_secs_f32(); // Move up
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= SPEED * time.delta().as_secs_f32(); // Move down
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= SPEED * time.delta().as_secs_f32(); // Move left
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += SPEED * time.delta().as_secs_f32(); // Move right
        }
    }
}

fn keep_above_ground(mut query: Query<&mut Transform, With<Object>>) {
    for mut transform in &mut query {
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
        }
    }
}

// Main and other components setup would go here

// Characters.
#[derive(Component)]
struct Object;

#[derive(Component)]
struct PlayableCharacter;

#[derive(Component)]
struct NonPlayableCharacter;

// Relations.
#[derive(Component)]
struct AlliedRelation;

#[derive(Component)]
enum CharacterFaction {
    RedFaction,
    BlueFaction,
    NeutralFaction,
}

#[derive(Component)]
struct NeutralRelation;

// Properties.
#[derive(Component)]
struct Name(String);
