struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn distance_between(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        (x * x + y * y).sqrt()
    }
}



fn main() {
    let point = Point::new(1.0, 2.0);
    println!("Point: ({}, {})", point.x, point.y);
    println!("Distance: {}", point.distance_between());

    println!("Hello, world!");
}
