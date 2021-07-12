fn main() {
    let str1 = String::from("1234");
    let str2 = "123456";
    let res = longest(&str1, &str2);
    println!("{}",res);
}

fn longest<'a>(x: &'a str,y:&'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}