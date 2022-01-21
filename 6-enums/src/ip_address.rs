#[derive(Debug)]
pub enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
pub struct IpAddr {
    kind: IpAddrKind,
    address: String
}

// Rather than using an enum inside a struct, we can put data directly into each enum variant
#[derive(Debug)]
pub enum IpAddr2 {
    V4(String),
    V6(String)
}

#[derive(Debug)]
pub enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

pub fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!("home is {:#?}", home);
    println!("loopback is {:#?}", loopback);
}

pub fn main2() {
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("home is {:#?}", home);
    println!("loopback is {:#?}", loopback);
}

pub fn main3() {
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    println!("home is {:#?}", home);
    println!("loopback is {:#?}", loopback);
}