fn main() {
    println!("rust_ownership");

    // 硬编码 tmp
    //let tmp = "tmp";
    //println!("{} is a string.",tmp); // tmp is a string.

    // String 类型
    // let mut tmp = String::from("hello");
    // tmp.push_str(",world!");
    // println!("{}",tmp);

    // 移动
    // let mv = String::from("MOVE");
    // println!("{}",mv);

    // let mv_2 = mv;
    // println!("{}",mv_2);

    // 变量与数据的交互：移动
    //println!("{}",mv); // error : value borrowed here after moverustc(E0382)

    // 克隆
    // let clone_1 = mv_2.clone();
    // println!("{}",clone_1);
    // println!("{}",mv_2);

    // 所有权与函数
    //let os_1 = gives_ownership();
    //let os_2 = String::from("hello");
    // os_2 -> os_3
    //let os_3 = takes_and_gives_back(os_2);

    // 引用与借用
    // let tmp = String::from("Hello,World!");
    // let len = calculate_length(&tmp);
    // println!("len is {}", len);

    // 使用可变引用
    // let mut tmp = String::from("Mut String");
    // change(&mut tmp);
    // println!("{}",tmp);

    // 在特定的作用域中的特定数据来说，一次只能声明一个可变引用，避免了数据竞争。
    //let mut tmp = String::from("abc");
    // // OK
    // let v1 = &mut tmp;

    // // cannot borrow `tmp` as mutable more than once at a time
    // let v2 = &mut tmp;

    // 切片(Slice)
    // eg. 编写一个搜索函数，接收字符串为参数，返回字符串中的第一个单词。如果字符串中不存在
    // 空格，则返回整个字符串

    // let mut str = String::from("Hello RUST!");
    // let fw = first_word(&str);

    // // output: Hello
    // println!("{}",fw);

    // // error: cannot borrow `str` as mutable because it is also borrowed as immutable
    // str.clear();
    // println!("{}",fw);
}

// first_word v1
fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn change(s: &String) {
//     //`s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//     //引用默认是不可变
//     s.push_str(",changed"); 
// }

// 使用可变引用
// 


// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn gives_ownership() -> String {
//     let some_thing = String::from("Hello");
//     some_thing
// }

// fn takes_and_gives_back(a_string:String) -> String {
//     a_string
// }
