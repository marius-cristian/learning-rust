enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

/// alternative, just enum without a struct;

enum IpAddrType2 {
    V4(String),
    V6(String),
}

fn route(_ip_kind: IpAddrKind) {}

// enum with many types

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// alternative structs for the Message enum
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("{:?}", "Lollerskater");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin2 {
    Quarter(UsState),
}

impl Coin2 {
    fn value_in_cents(coin: Coin2) -> u8 {
        match coin {
            Coin2::Quarter(state) => {
                println!("{:?}", state);
                25
            }
        }
    }
}
fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home_type2 = IpAddrType2::V4(String::from("127.0.0.1"));
    let _loopback_type2 = IpAddrType2::V6(String::from("::1"));

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let m = Message::Write(String::from("hello"));
    m.call();
    println!("Hello, world!");

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = None; //Some(5);

    let sum = x + y.unwrap_or(0);
    let prod = x * y.unwrap_or(1);

    println!("x:{:?} y:{:?} sum:{:?} prod:{:?}", x, y, sum, prod);

    println!("Coin: {:?}", value_in_cents(Coin::Penny));

    println!("Coin: {:?}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}
