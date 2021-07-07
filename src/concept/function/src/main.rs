fn main() {
    println!("Hello, world!");

    // 1. 块最后的表达式代表块的值
    let x = {
        let y = 3;
        y + 4
    };

    println!("the value of x : {}", x);

    // 2.
    let y = five();
    println!("the value of y : {}", y);

    // 3.
    let z = plus(10);
    println!("the value of z : {}", z);

    // 4. if...else
    let num = 100;
    if num > 200 {
        println!("{} > 200", num);
    } else if num >= 100 {
        println!("{} >= 100", num);
    } else {
        println!("done!");
    }

    // 5.loop
    let mut res : i32 = 100;
    let loop_reslut = loop {
        if res >= 300 {
            break res + 1
        }

        res += 20;
    };
    println!("loop reslt : {}", loop_reslut);

    // 6. array
    let for_array = [10,20,30,40,50,60];
    for el in for_array.iter(){
        println!("value : {}",el);
    }

    // 7. Range
    for num in (1..4).rev() {
        println!("value : {}",num);
    }
}

fn five() -> i32 {
    5
}

fn plus(x: i32) -> i32 {
    x + 5
}
