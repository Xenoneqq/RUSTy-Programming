use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
struct Vec2D {
    x: f32,
    y: f32,
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


fn main() {
    let mut vector = Vec2D { x: 10.0, y: 5.0 };
    let op = Vec2D { x: 2.0, y: 2.0 };

    vector = vector + op;
    println!("After + : {} {}", vector.x, vector.y);

    vector = vector - op;
    println!("After - : {} {}", vector.x, vector.y);
}
