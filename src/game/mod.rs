mod puzzle;
use bevy::prelude::*;
use puzzle::{Puzzle, Selector};

const SELECTOR_LAYER: f32 = 2.0;
const SUI_BLUE: Color = Color::srgba_u8(77, 162, 255, 64);
const ERROR_RED: Color = Color::srgba_u8(209, 145, 145, 128);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, conflict_check)
            .add_systems(Update, puzzle_control.after(conflict_check));
    }
}
fn puzzle_control(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cursor: Query<(&mut Transform, &mut Selector)>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    if let Ok((mut transform, mut cursor)) = cursor.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Enter) || keyboard_input.pressed(KeyCode::KeyC) {
            if cursor.conflict {
                commands.spawn(AudioPlayer::new(asset_server.load("audio/error_008.ogg")));
            } else {
                let drop_sound = AudioPlayer::new(asset_server.load("audio/drop_003.ogg"));
                for p in cursor.drop().into_iter() {
                    commands.spawn((
                        Sprite::from_image(asset_server.load("img/block_07.png")),
                        p,
                        Puzzle {},
                    ));
                }
                commands.spawn(drop_sound);
            }
            wait_key_release(200);
        }

        if keyboard_input.pressed(KeyCode::KeyR) {
            let quat = cursor.rotate();
            *transform = transform.with_rotation(quat);
            commands.spawn(AudioPlayer::new(
                asset_server.load("audio/question_004.ogg"),
            ));
            wait_key_release(200);
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
        let new_pos = transform.translation + direction * Selector::step();
        let half_image_size = Selector::width() / 2.0;
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
            wait_key_release(100);
        }
    }
}

// Check there is puzzle under selector
fn conflict_check(
    mut cursor: Query<(
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut Selector,
    )>,
    puzzle_query: Query<&Transform, (With<Puzzle>, Without<Selector>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((transform, mut mesh_material, mut cursor)) = cursor.single_mut() {
        let puzzle_pos = puzzle_query.iter().map(|p| p.translation).collect();
        let orig_conflict = cursor.conflict;
        cursor.check_conflict(
            Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            },
            puzzle_pos,
        );

        if orig_conflict != cursor.conflict {
            let color = if cursor.conflict { ERROR_RED } else { SUI_BLUE };
            // TODO: there should be a better way for switch color
            *mesh_material = MeshMaterial2d(materials.add(color));
        }
    }
}

fn wait_key_release(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

fn setup(
    mut commands: Commands,
    window: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window_size = window.resolution.physical_size().as_vec2();
    println!("window size: {}", window_size);

    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(Selector::width(), Selector::width()))),
        MeshMaterial2d(materials.add(SUI_BLUE)),
        Transform::from_xyz(0., 0., SELECTOR_LAYER),
        Selector {
            puzzles: vec![0, 1, 2, 3, 4],
            ..Default::default()
        },
    ));
}
