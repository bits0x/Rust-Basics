use std::collections::HashMap;

fn main() {
    // Vectors
    let arr = [1, 2, 3];
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // access element of vector
    let third = &v1[2];
    println!("{}", third);
    
    // or safer way is use get
    match v1.get(2) {
        Some(ele) => println!("{}", ele),
        None => println!("index out of bound")
    }
    
    let v2 = vec![1, 2, 3];
    for item in &v2 {
        println!("{}", item);
    }

    // Strings : They are stored as a collection of UTF-8 encoded bytes

    let s1 = String::new();
    let s2 = "string slice";
    let s3 = s2.to_string();
    let s4 = String::from("string");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    // concatenation 
    let s5 = s4 + &s; // stringfoobar!
    
    // or use format macro
    let s6 = format!("{}{}", s5, s); // stringfoobar!foobar!
    println!("{}{}", s5, s6);

    // read bytes of string
    for i in s.bytes() {
        println!("{}", i);
    }

    // read chars of string
    for i in s.chars() {
        println!("{}", i);
    }

    // Hash-maps

    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    // Accessing
    let key = String::from("blue");
    let blue_team_score = scores.get(&key); // 10

    match blue_team_score {
        Some(value) => println!("{}", value),
        _ => println!("Invalid key")
    }

    for (key, value) in &scores {
        println!("key -> {} , value -> {}", key , value);
    }

    let text = "hello world wonderful world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; 
    }

    /* 
        { 
            "wonderful": 1,
            "world": 2,
            "hello": 1,
        }
    */ 
    println!("{:#?}", map); 
}