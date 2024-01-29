
struct Rectangle { // Declare a struct
    width: f32,
    height: f32
}

fn main() {
    let rect = Rectangle {
        width: 30.0,
        height: 50.5
    };

    // Print fields of struct using dot notation
    println!("width = {}, height = {}", rect.width, rect.height);

    let area = area(&rect);
    println!("Area = {} pixels", area);
}

fn area(rectangle: &Rectangle) -> f32 {
    rectangle.width * rectangle.height
}

