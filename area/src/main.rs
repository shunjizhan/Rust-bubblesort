use std::f64::consts::PI;

struct Circle { radius: f64 }
struct Triangle { base: f64, height: f64 }
struct Square { side: f64 }

trait Area { fn area(&self) -> f64; }

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Area>(name: &str, shape: &T) {
    println!("{}'s area is: {}", name, shape.area());
}

fn main() {
    let circle = Circle { radius: 1.0 };
    let triangle = Triangle { base: 0.4, height: 3.2 };
    let square = Square { side: 1.0 };

    print_area("circle", &circle);
    print_area("triangle", &triangle);
    print_area("squre", &square);
}
