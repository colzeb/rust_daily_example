#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrMulti {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Write(x) => println!("{}", x),
            _ => println!("Not string"),
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);
    println!("{:?}", six);

    let home = IpAddr::V4("192.168.1.1".to_string());
    let office = IpAddr::V6("::1".to_string());

    println!("{:?}", home);
    println!("{:?}", office);

    let home = IpAddrMulti::V4(127, 0, 0, 1);
    let loopback = IpAddrMulti::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    match home {
        IpAddrMulti::V4(_, x, _, _) => println!("second element in v4 {}", x),
        IpAddrMulti::V6(x) => println!("V6 of ip {}", x),
    }

    let move_x = Message::Move { x: 45, y: 30 };
    println!("Move x {:?}", move_x);
    move_x.call();
    let hello_enum = Message::Write("Welcome".to_string());
    hello_enum.call();
}
