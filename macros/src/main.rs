use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let _v1: Vec<u32> = vec![1, 2, 3];

    let _v2: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    //---------------------------------------------
    Pancakes::hello_macro();
    
}
