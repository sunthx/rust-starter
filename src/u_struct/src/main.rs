fn main() {
    println!("Hello, world!");

    // // 声明一个 User，但不可变
    // let user = User {
    //     email: String::from("sunth0101@gmail.com"),
    //     username: String::from("sunthx"),
    //     sign_in_count: 1,
    //     active: true,
    // };

    // // 声明一个可变的 User
    // let mut user_mut = User {
    //     email: String::from("sunth0101@gmail.com"),
    //     username: String::from("sunthx"),
    //     sign_in_count: 1,
    //     active: true,
    // };

    // user_mut.email = String::from("sunthx@outlook.com");

    // // 通过 build_user 声明一个 user
    // let user2 = build_user(String::from("email"), String::from("username"));
    // print_user(user2);

    // // 根据其他实例的值创建新的实例
    // // 使用 user1 中的部分字段值创建 user2
    // let user1 = build_user(String::from("user1@mail.com"),String::from("user1"));
    // let user2 = User {
    //     username: String::from("user2"),
    //     email: String::from("user2@mail.com"),
    //     //其他的值使用 user1
    //     ..user1
    // };

    // print_user(user2);

    // // 元组结构体的使用
    // let p1 = Point(0,0,0);
    // let p2 = Point(1,1,1);

    // eg. 计算一个长方形的面积
    let rect = Rectangle {
        width:30,
        height:40
    };

    let rect1 = Rectangle {
        width: 20,
        height: 30
    };

    // let area = area(&rect);
    // let area = rect.area();
    // let can_hold = rect.can_hold(&rect1);
    // println!("rect can hold rect1 : {}", can_hold);
    // println!("area: {}",area);
    // println 默认是 Display 的格式化输出
    // 标识符 :? 代表使用 DEBUG 的格式输出
    // println!("rect is {:?}", rect);
    // println!("rect is {:#?}", rect);

    // 关联函数
    // 主要用于构造函数
    // 使用 :: 使用关联函数
    // let square = Rectangle::square(20);
}

// 声明一个 struct
struct User {
    username: String,
    email: String,
    sign_in_count : u64,
    active: bool,
}

// * 元组结构体
struct Point(i32,i32,i32);
struct Color(i32,i32,i32);


fn build_user(email:String, username:String) -> User {
    User {
        // 使用 字段初始化简写 (field init shorthand)
        // email: email
        // username: username
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}

fn print_user(user:User){
    println!("email: {}",user.email);
    println!("username: {}",user.username);
    println!("active: {}",user.active);
    println!("sign_in_count: {}",user.sign_in_count);
}

// 定义一个表示长方形的结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// // 将 area 函数改为 Rectangle 中的方法
// fn area(rect: &Rectangle) -> u32{
//     rect.width * rect.height
// }

// 使用  impl 关键字
// 将 area 定义为 Rectangle 的方法
impl Rectangle {
    fn square(len:u32) -> Rectangle {
        Rectangle {
            width: len,
            height: len,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self,rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}