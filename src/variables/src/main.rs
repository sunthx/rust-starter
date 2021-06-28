// 1. 常量与变量
// 2. 隐藏（Shadowing)
fn main() {
    println!("Hello, world!");

    // let guess : i32 = "42".parse().expect("error");
    // println!("value {}",guess);

    // -标量类型
    
    // -复合类型
    // 1. tuple
    let t:(i32,i32,i32) = (1,2,3);
    println!("1-{} 2-{} 3-{}",t.0,t.1,t.2);

    let (x1,x2,x3) = t;
    println!("1-{} 2-{} 3-{}",x1,x2,x3);

    // 2. array
    let a = [1,2,3,4,5];
    println!("1-{}",a[0]);
}
