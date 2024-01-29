fn main() {
    
    let mut i = 1;
    let value = loop {
        if i > 10 {
            break i; // break & return a value from loop
        }
        i += 1;
    };
    assert_eq!(value, 11); // i = 11

    let mut counter = 0;

    while counter < 10 {
        println!("hello");
        counter = counter + 1;
    }

    for item in 0..5 { // 5 exclusive
        println!("{}", item*2);
    }
}