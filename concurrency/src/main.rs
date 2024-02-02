use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};


fn main() {
    // Concurrency : Creating threads
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

    //----------------------------------------------------
    // Concurrency : Message passing
    let (tx, rx) = mpsc::channel(); 

    let tx1 = tx.clone(); // creating another transmitter channel

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    //------------------------------------------
    // Shared State concurrency

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
