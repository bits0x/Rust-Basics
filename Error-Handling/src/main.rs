use std::fs::File;

fn main() {
    // panic!("crash and burn");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Probelm opening the file {:?}", error)
    };
    
    // or use unwrap method
    let mut f1 = File::open("hello.txt").unwrap();

}

 