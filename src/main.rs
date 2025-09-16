use bevy::prelude::*;

const MOVE_SPEED: f32 = 64.0; // Image size

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, puzzle_control)
        .run();
}

#[derive(Clone, Default, Debug)]
enum Rotation {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Rotation {
    fn rotate(&mut self) -> Quat {
        let angle = self.angle();
        match self {
            Rotation::Up => {
                *self = Rotation::Right;
            }
            Rotation::Right => {
                *self = Rotation::Down;
            }
            Rotation::Down => {
                *self = Rotation::Left;
            }
            Rotation::Left => {
                *self = Rotation::Up;
            }
        }
        angle
    }
    fn angle(&self) -> Quat {
        match self {
            Rotation::Up => Quat::from_rotation_z(0.0 * std::f32::consts::PI),
            Rotation::Right => Quat::from_rotation_z(0.5 * std::f32::consts::PI),
            Rotation::Down => Quat::from_rotation_z(1.0 * std::f32::consts::PI),
            Rotation::Left => Quat::from_rotation_z(-0.5 * std::f32::consts::PI),
        }
    }
}

#[derive(Component, Default, Clone)]
struct Puzzle {
    rotation: Rotation,
}

#[derive(Component)]
struct PlacedPuzzle {
    rotation: Rotation,
}

fn puzzle_control(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut puzzle_query: Query<(&mut Transform, &mut Puzzle)>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut transform, mut puzzle)) = puzzle_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Enter) || keyboard_input.pressed(KeyCode::KeyC) {
            let place = transform.translation;
            commands.spawn((
                Sprite::from_image(asset_server.load("block_07.png")),
                Transform::from_xyz(place.x, place.y, place.z)
                    .with_rotation(puzzle.rotation.angle()),
                PlacedPuzzle {
                    rotation: puzzle.rotation.clone(),
                },
            ));
        }

        if keyboard_input.pressed(KeyCode::KeyR) {
            let quat = puzzle.rotation.rotate();
            *transform = transform.with_rotation(quat);
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * MOVE_SPEED;
    }
}

fn setup(mut commands: Commands, window: Single<&Window>, asset_server: Res<AssetServer>) {
    let window_size = window.resolution.physical_size().as_vec2();
    println!("window size: {}", window_size);

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("block_07.png")),
        Transform::from_xyz(0., 0., 0.),
        Puzzle::default(),
    ));

    // Create a minimal UI explaining how to interact with the example
    // commands.spawn((
    //     Text::new(
    //         "Move the mouse to see the circle follow your cursor.\n\
    //                 Use the arrow keys to move the camera.\n\
    //                 Use the comma and period keys to zoom in and out.\n\
    //                 Use the WASD keys to move the viewport.\n\
    //                 Use the IJKL keys to resize the viewport.",
    //     ),
    //     Node {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(12.0),
    //         left: Val::Px(12.0),
    //         ..default()
    //     },
    // ));
}
