use bevy::prelude::*;
const IMAGE_SIZE: f32 = 64.0; // Image size

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, puzzle_control)
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
        self.angle()
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
    placed_puzzle_query: Query<&Transform, (With<PlacedPuzzle>, Without<Puzzle>)>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    if let Ok((mut transform, mut puzzle)) = puzzle_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Enter) || keyboard_input.pressed(KeyCode::KeyC) {
            let place = transform.translation;
            let mut conflict = false;
            for t in placed_puzzle_query.iter() {
                if t.translation.x == place.x && t.translation.y == place.y {
                    conflict = true;
                }
            }
            if conflict {
                commands.spawn(AudioPlayer::new(asset_server.load("audio/error_008.ogg")));
            } else {
                let drop_sound = AudioPlayer::new(asset_server.load("audio/drop_003.ogg"));
                commands.spawn((
                    Sprite::from_image(asset_server.load("img/block_07.png")),
                    Transform::from_xyz(place.x, place.y, 0.0)
                        .with_rotation(puzzle.rotation.angle()),
                    PlacedPuzzle {
                        rotation: puzzle.rotation.clone(),
                    },
                ));
                commands.spawn(drop_sound);
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        if keyboard_input.pressed(KeyCode::KeyR) {
            let quat = puzzle.rotation.rotate();
            *transform = transform.with_rotation(quat);
        }
        let mut x_movement = true;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            x_movement = false;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            x_movement = false;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        let window = window.resolution.physical_size().as_vec2();
        let new_pos = transform.translation + direction * IMAGE_SIZE;
        let half_image_size = IMAGE_SIZE / 2.0;
        let half_window_width = window.x / 2.0;
        let half_window_height = window.y / 2.0;
        if (x_movement
            && new_pos.x >= half_image_size - half_window_width
            && new_pos.x <= half_window_width - half_image_size)
            || (!x_movement
                && new_pos.y >= half_image_size - half_window_height
                && new_pos.y <= half_window_height - half_image_size)
        {
            transform.translation = new_pos;
        }
    }
}

fn setup(mut commands: Commands, window: Single<&Window>, asset_server: Res<AssetServer>) {
    let window_size = window.resolution.physical_size().as_vec2();
    println!("window size: {}", window_size);

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("img/block_07.png")),
        Transform::from_xyz(0., 0., 1.),
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
