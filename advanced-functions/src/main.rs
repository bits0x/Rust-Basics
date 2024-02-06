fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is, {}!", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Or we could name a function as the argument to map instead of the closure, like this:
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("The answer is, {:?}!", list_of_statuses);

    let closure = return_closure();
    let result = closure(5);
    println!("Result: {}", result); // This will print "Result: 6"

}

// Returning Closures
fn return_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}


