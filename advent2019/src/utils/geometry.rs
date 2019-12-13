use std::ops;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Default)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Vec2D {
    pub fn distance_between(&self, other: &Self) -> f64 {
        (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)) as f64)
            .sqrt()
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

pub enum CardinalDirection {
    North,
    East,
    South,
    West,
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

pub fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x.abs();
    let mut y = y.abs();
    if x == 0 || y == 0 {
        return 0;
    }
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }

    x
}

pub fn lcm(x: i64, y: i64) -> i64 {
    let gcd = gcd(x, y);
    if gcd <= 0 {
        0
    } else {
        x / gcd * y
    }
}
