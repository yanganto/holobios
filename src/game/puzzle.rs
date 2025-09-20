use bevy::prelude::*;

const IMAGE_SIZE: f32 = 64.0; // Image size
const PUZZLE_LEVEL: u8 = 2;
const PUZZLE_LAYER: f32 = 1.0;

#[derive(Component, Default, Clone)]
pub struct Selector {
    pub rotation: Rotation,
    pub conflict: bool,
    pub pos: Vec2,
    /// store needed puzzle by puzzle_idx start form 0 to 2 * PUZZLE_LEVEL - 1
    pub puzzles: Vec<u8>,
}

impl Selector {
    #[inline]
    pub fn width() -> f32 {
        IMAGE_SIZE + (PUZZLE_LEVEL - 1) as f32 * 2.0 * IMAGE_SIZE
    }

    #[inline]
    pub fn step() -> f32 {
        IMAGE_SIZE
    }

    pub fn check_conflict(&mut self, current_pos: Vec2, puzzle_pos: Vec<Vec3>) {
        self.conflict = true;
        self.pos = current_pos;
        let x_min = self.pos.x - Self::width() / 2.0;
        let x_max = self.pos.x + Self::width() / 2.0;
        let y_min = self.pos.y - Self::width() / 2.0;
        let y_max = self.pos.y + Self::width() / 2.0;

        for p in puzzle_pos.into_iter() {
            if p.x > x_min && p.x < x_max && p.y > y_min && p.y < y_max {
                return;
            }
        }
        self.conflict = false;
    }

    pub fn drop(&self) -> Vec<Transform> {
        let mut out = Vec::new();
        for i in 1..(2 * PUZZLE_LEVEL) {
            let i_idx = i as f32 - PUZZLE_LEVEL as f32;
            for j in 1..(2 * PUZZLE_LEVEL) {
                let j_idx = j as f32 - PUZZLE_LEVEL as f32;

                let puzzle_idx = (i - 1) * (2 * PUZZLE_LEVEL - 1) + j - 1;
                if self.puzzles.contains(&puzzle_idx) {
                    out.push(
                        Transform::from_xyz(
                            self.pos.x + i_idx * IMAGE_SIZE,
                            self.pos.y + j_idx * IMAGE_SIZE,
                            PUZZLE_LAYER,
                        )
                        .with_rotation(self.rotation.angle()),
                    );
                }
            }
        }
        out
    }
    pub fn rotate(&mut self) -> Quat {
        let rotated_puzzles = self.puzzles.iter().map(idx_map).collect();
        self.puzzles = rotated_puzzles;
        self.rotation.rotate()
    }
}

fn idx_map(i: &u8) -> u8 {
    match PUZZLE_LEVEL {
        2 => match i {
            0 => 2,
            1 => 5,
            2 => 8,
            3 => 1,
            4 => 4,
            5 => 7,
            6 => 0,
            7 => 3,
            8 => 6,
            _ => panic!("Error number in puzzles"),
        },
        1 => *i,
        _ => panic!("Unsupported puzzle Level"),
    }
}

#[derive(Component)]
pub struct Puzzle {}

#[derive(Clone, Default, Debug)]
pub enum Rotation {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Rotation {
    pub fn rotate(&mut self) -> Quat {
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
    pub fn angle(&self) -> Quat {
        match self {
            Rotation::Up => Quat::from_rotation_z(0.0 * std::f32::consts::PI),
            Rotation::Right => Quat::from_rotation_z(0.5 * std::f32::consts::PI),
            Rotation::Down => Quat::from_rotation_z(1.0 * std::f32::consts::PI),
            Rotation::Left => Quat::from_rotation_z(-0.5 * std::f32::consts::PI),
        }
    }
}
