enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields like a struct
    Write(String),              // Contains a String
    ChangeColor(i32, i32, i32), // Contains three i32 values
}

//Just like structs, you can define methods on enums:

impl Message {
    fn call(&self) {
        // Method implementation
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    route(four);
    route(six);

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(_ip_kind: IpAddrKind) {
    // Function body can handle both V4 and V6
}