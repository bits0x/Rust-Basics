#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_fn() {
        println!("Enums in Rust");
    }
}

fn main() {
    let message1 = Message::Quit;
    let message2 = Message::Move{x:-1, y: 1};
    let message3 = Message::Write(String::from("Hello"));
    let message4 = Message::ChangeColor(1,2,3);
    Message::some_fn();
    println!("{:#?} {:#?} {:#?} {:#?}", match_enum(message1), match_enum(message2), match_enum(message3), match_enum(message4));

    // The option enum
    let x = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = None;

    let sum1 = x + y.unwrap_or(0); // sum = 10
    let sum2 = x + z.unwrap_or(0); // sum = 0 as z takes default value 0

    println!("sum1 = {}, sum2 = {}", sum1, sum2);

}

fn match_enum(msg: Message) -> i32 {
    match msg {
        Message::Quit => 1,
        Message::Move{x, y} => x + y,
        Message::Write(str) => {
            println!("{}", str);
            0
        },
        Message::ChangeColor(a, b, c) => a + b + c
     }
}
