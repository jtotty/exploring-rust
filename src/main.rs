mod shapes;

use crate::shapes::{circle::Circle, rectangle::Rectangle, area::Area};

fn main() {
    let rect = Rectangle::default();

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{:?}", circ.area());
    println!("{}", rect);
}
