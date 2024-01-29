fn main() {

    // Scalar types like integers, float, bool have a copy trait which can help to make a copy of it 
    let x = 5;
    let y = x; // Copy

    // ----------------------------------

    let mut var1 = String::from("Ownership");

    var1.push_str("rust");

    let var2 = var1; // ownership is changed to var2

    // This line will return an error as at a time in rust no two variables can 
    // own or points to single location refernce 
    // println!("{}", var1); 

    println!("{}", var2);

    // -------------------------------

    // To copy compound types like string we need to use clone method instead
    let var3 = String::from("Ownership");

    let var4 = var3.clone();

    println!("{}", var3); // No error as we have used expensive clone method to make a copy

}