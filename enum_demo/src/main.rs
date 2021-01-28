#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Hello, enum {:#?}", Message::ChangeColor(255,255,255));
    }
}


fn main() {
    println!("Hello, enum {:#?}", Message::Quit);
    let m = Message::ChangeColor(25,25,25);
    m.call();

    let some_number = Some(5);   
    // Can't plus number, different type
    // let plus_option_number = some_number + 5;
    println!("Hello, Option enum value: {:#?}", some_number.unwrap());

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("match any"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
