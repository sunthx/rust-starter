// demo: https://www.bilibili.com/video/BV1hp4y1k7SV?p=7
// 使用 use 关键字引用库
// std 标准库
use std::io;

// 生成随机数 https://crates.io/crates/rand
use rand::Rng; // trait

use std::cmp::Ordering;

fn main() {
    println!("guessing");

    let mut rng = rand::thread_rng();
    let secret_num = rng.gen_range(1..101);
    println!("current secret num: {}", secret_num);

    // 1.使用 let 声明变量
    // 2.默认是不可变的(immutable)
    //let tmp = 1;
    //tmp = 2; // err:cannot assign twice to immutable variable `tmp`

    // 1. 使用 mut 关键字表示该变量是可变的。
    // let mut tmp2 = 1;
    // tmp2 = 2; // ok

    // :: 表示关联函数,调用类似 C# 里的静态函数 StaticClass.Method();

    // 循环
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error");
        println!("output {}", guess);

        // 显式声明一个变量
        // 使用 match 处理错误
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
