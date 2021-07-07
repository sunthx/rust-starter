fn main() {
    println!("Hello, world!");

    // let four = IpAddrKind::V4;
    // println!("{:#?}", four);

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // println!("{:#?}", home);
    // println!("{:#?}", loopback);

    // match 模式匹配
    let coin = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", coin)

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// 定义一个枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 枚举允许直接将关联的数据嵌入到枚举变体内
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// ** 神奇的枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message::Call");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        },
        // 使用 _ 通配符忽略不需要处理的分支
        _ => 0
    }
}

// 使用 if let 语法糖代替 match 
// 只处理 penny 变体
fn penny_value_in_cents(coin: Coin) -> u32 {
    if let coin = Coin::Penny {
        1
    }
    else {
        2
    }
}

// Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
