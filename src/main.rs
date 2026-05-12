#[derive(Debug)]
enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
    Hsl(u8, u8, u8),
    Cmyk(u8, u8, u8, u8),
    Gray(u8),
}


struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Point {
    fn distance_between(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
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

    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h,
        }
    }
    // fn set_to_max(&mut self, other: Rectangle) {
    //     *self = self.max(other);
    // }
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

    let cloud = Color::Rgb(255, 255, 255);
    let grass = Color::Hsv(100, 100, 100);
    let water = Color::Hsl(100, 100, 100);
    let earth = Color::Cmyk(100, 100, 100, 100);
    let sky = Color::Gray(100);
    println!("Cloud -> {:?}", cloud);
    println!("Sky -> {:?}", sky);
    println!("Grass -> {:?}", grass);
    println!("Water -> {:?}", water);
    println!("Earth -> {:?}", earth);
    println!("Sky -> {:?}", sky);

    println!("Cloud -> {:?}", match cloud {
        Color::Rgb(r, g, b) => format!("RGB({r}, {g}, {b})"),
        Color::Hsv(h, s, v) => format!("HSV({h}, {s}, {v})"),
        Color::Hsl(h, s, l) => format!("HSL({h}, {s}, {l})"),
        Color::Cmyk(c, m, y, k) => format!("CMYK({c}, {m}, {y}, {k})"),
        Color::Gray(g) => format!("Gray({g})"),
    });

}
