mod shapes;

use shapes::collisions::Collidable;

use crate::shapes::{circle::Circle, rectangle::Rectangle};

fn main() {
    let rect1 = Rectangle::default();
    let rect2 = Rectangle::default();

    let circle1 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };

    let circle2 = Circle {
        x: 1.5,
        y: 1.5,
        radius: 4.0,
    };

    let result1 = rect1.collide(&rect2);
    let result2 = circle1.collide(&circle2);
    let result3 = rect1.collide(&circle1);

    println!("{}", result1);
    println!("{}", result2);
    println!("{}", result3);
}
