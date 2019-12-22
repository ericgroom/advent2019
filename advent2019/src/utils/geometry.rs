use std::collections::HashMap;
use std::ops;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Default)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Vec2D {
    pub fn euclidean(&self, other: &Self) -> f64 {
        (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)) as f64)
            .sqrt()
    }

    pub fn manhattan(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn new(x: i32, y: i32) -> Vec2D {
        Vec2D { x: x, y: y }
    }
}

impl std::fmt::Debug for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl ops::Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2D::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Default)]
pub struct Vec3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3D {
    pub fn new(x: i32, y: i32, z: i32) -> Vec3D {
        Vec3D { x: x, y: y, z: z }
    }
}

impl std::fmt::Debug for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn clockwise(&self) -> CardinalDirection {
        match &self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn counter_clockwise(&self) -> CardinalDirection {
        match &self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

pub fn render_image(
    image: Vec<i32>,
    row_width: usize,
    pixel_renderer: Box<dyn Fn(&i32) -> char>,
) -> String {
    let mut result = String::new();
    for row in image.chunks(row_width) {
        for pixel in row {
            let c = pixel_renderer(pixel);
            result.push(c)
        }
        result.push('\n')
    }
    result
}

pub fn convert_map_to_grid<T: Copy>(
    map: &HashMap<Vec2D, T>,
    empty_value: i32,
    transform: Box<dyn Fn(T) -> i32>,
) -> (usize, Vec<i32>) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    for point in map.keys() {
        if point.x < min_x {
            min_x = point.x
        }
        if point.x > max_x {
            max_x = point.x
        }
        if point.y < min_y {
            min_y = point.y
        }
        if point.y > max_y {
            max_y = point.y
        }
    }
    let y_shift = if min_y < 0 { -min_y } else { 0 };
    let x_shift = if min_x < 0 { -min_x } else { 0 };
    let width = max_x - min_x;
    let mut result: Vec<i32> = Vec::new();
    result.resize(
        ((max_x + x_shift + 1) * (max_y + y_shift + 1)) as usize,
        empty_value,
    );
    for (point, value) in map {
        let index = (width * (point.y + y_shift) + (point.x + x_shift)) as usize;
        result[index] = transform(*value);
    }
    (width as usize, result)
}
