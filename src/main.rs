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

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let point = Point::new(1.0, 2.0);
    println!("Point: ({}, {})", point.x, point.y);
    println!("Distance: {}", point.distance_between());

    println!("Hello, world!");

    let mut rect = Rectangle::new(10, 20);
    let rect2 = Rectangle::new(30, 40);
    println!("Rectangle 1 -> ({}, {})", rect.width, rect.height);
    println!("Area 1 -> {}", rect.area());
    rect.set_width(30);
    println!("Rectangle 2 -> ({}, {})", rect.width, rect.height);
    println!("Area 2 -> {}", rect.area());

    let max_rect = rect.max(rect2);
    println!("Max Rectangle -> ({}, {})", max_rect.width, max_rect.height);
    println!("Area of max rectangle -> {}", max_rect.area());
}
