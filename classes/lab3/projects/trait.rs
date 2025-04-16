use std::fmt::{Display, Formatter};

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;

    fn describe(&self) {
        println!("I am a general shape!");
        println!("Area : {}", self.area());
        println!("Perimeter : {}", self.perimeter());
    }
}

struct Rectangle {
    x: f32,
    y: f32
}

impl Shape for Rectangle {
    fn area(&self) -> f32{
        self.x * self.y
    }

    fn perimeter(&self) -> f32{
        2.0 * (self.x + self.y)
    }

    fn describe(&self) {
        println!("I am a rectangle!");
        println!("Area : {}", self.area());
        println!("Perimeter : {}", self.perimeter());
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle[x={},y={}]", self.x, self.y)?;
        Ok(())
    }
}

fn main(){
    let rect = Rectangle{x: 2.0,y: 5.0};
    rect.describe();
    println!("---");
    println!("{}", rect);
}