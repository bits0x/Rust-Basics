use std::thread;
use std::time::Duration;

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} in spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        };
    });

    for i in 1..5 {
        println!("{} in main thread", i);
        thread::sleep(Duration::from_millis(1));
    };
   
    handle.join().unwrap();

    let v = vec![1, 3, 5];

    // use move keyword to tranfer the ownership of v to closure 
    let handle2 = thread::spawn(move || {
        println!("value of v are: {:?}", v);
    });

    handle2.join().unwrap();

}
