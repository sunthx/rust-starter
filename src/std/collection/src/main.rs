use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // 通用集合类型
    // 1. Vector
    // 2. String
    // 3. Hashmap

    let mut v: Vec<i32> = Vec::new();
    // 使用 宏vec! 创建
    // let v = vec![1,2,3];
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let third = &v[2];
    println!("third value is {}", third);

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let multi_type_values = vec![
        SpreadsheetCell::Float(2.0),
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("Text")),
    ];

    for i in &multi_type_values {
        println!("{:?}", i);
    }

    // 2. String
    let data = "foo";

    // s: String
    let mut s = data.to_string();
    s.push_str("bar");
    println!("s is {}", s);
    let str1 = "str1";
    let str2 = "str2";
    let str3 = "str3";

    // 使用 format！ 宏组合字符串
    let res = format!("{}-{}-{}", str1, str2, str3);
    println!("res is {}", res);
    for i in res.chars() {
        println!("{}", i);
    }

    for i in res.bytes() {
        println!("{}", i);
    }

    // 3. Hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("sunfei"), 100);
    scores.insert(String::from("wangxiaofei"), 99);
    scores.entry(String::from("erdan")).or_insert(59);

    let names = vec![String::from("sunfei"), String::from("wangxiaofei")];
    let scores = vec![100, 99];

    let map: HashMap<_, _> = names.iter().zip(scores.iter()).collect();

    for (key, value) in &map {
        println!("{}-{}", key, value);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
