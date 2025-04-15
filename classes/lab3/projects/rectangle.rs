struct Rectangle {
    x : f32,
    y : f32
}

impl Rectangle {
    fn new_square(x : f32) -> Rectangle {
        Rectangle{x, y : x}
    }

    fn area(&self) -> f32 {
        self.x * self.y
    }

    fn circumference(&self) -> f32 {
        self.x * 2.0 + self.y * 2.0
    }
}

fn main(){
    let square = Rectangle::new_square(10.0);
    println!("Area: {}", square.area());
    println!("Circumference: {}", square.circumference());

    let rect = Rectangle{x: 4.0, y: 2.0};
    println!("Area: {}", rect.area());
    println!("Circumference: {}", rect.circumference());
}