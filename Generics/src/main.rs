struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20};
    let p2 = Point { x: 10.22, y: 20.05};
    let p3 = Point { x: 10, y: 20.15};
    let p4 = Point { x: "hello", y: "world"};

    let p5 = p3.mixup(p4); // {x: 10 , y: world}
    
    let v = vec![10, 2, 3, 42];

    let largest = get_largest(v);

    let v2 = vec!['q', ' ', 'c' , 'v'];

    let largest2 = get_largest(v2);
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}