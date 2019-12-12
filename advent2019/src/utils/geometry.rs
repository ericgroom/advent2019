#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn distance_between(&self, other: &Self) -> f64 {
        (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)) as f64)
            .sqrt()
    }

    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

pub fn render_image(image: Vec<i32>, row_width: usize) -> String {
    let mut result = String::new();
    for row in image.chunks(row_width) {
        for pixel in row {
            let c = match pixel {
                0 => '█',
                1 => ' ',
                2 => 't',
                _ => panic!("pixel other than 0, 1, 2"),
            };
            result.push(c)
        }
        result.push('\n')
    }
    result
}
