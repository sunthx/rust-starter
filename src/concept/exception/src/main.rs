use std::fs::File;
use std::io::Read;
use std::io;

fn main() {
    println!("Hello, world!");

    // // Output:
    // // Hello, world!
    // // thread 'main' panicked at 'crash and burn', src\main.rs:4:5
    // // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // // error: process didn't exit successfully: `target\debug\exception.exe` (exit code: 101)
    // panic!("crash and burn");


    // //RUST_BACKTRACE= 1 | full
    // //显示 panic 的详细堆栈信息
    // let v = vec![1,2,3];
    // let i = v[99];

    // //可恢复错误 Result 类型
    // //比如打开一个文件 a.txt
    // // file 是一个 Result<File,Error>
    // let file = File::open("a.txt"); 

    // // 使用 match 表达式处理 Result 类型
    // let res = match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("a.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("tried to create file but there was a problem:{:?}",e),
    //         },
    //         other_error => panic!("There was a problem opening the file:{:?}",other_error), 
    //     },
    // };

    // 使用 ? 操作符完成错误传播
    let name = read_username_from_file();
}

fn read_username_from_file() -> Result<String,io::Error> {
    // 1.使用 match 表达式实现
    // let f = File::open("1.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(val) => Ok(val),
    //     Err(e) => return Err(e),
    // };

    // 2.使用 ? 操作符简化代码
    // let mut f = File::open("1.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // // 3.继续简化
    // let mut res = String::new();
    // File::open("1.txt")?.read_to_string(&mut res)?;
    // Ok(res)

    // 4. 为了简化
    std::fs::read_to_string("1.txt")
}