fn main() {
    println!("Hello, world!");

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let res = largest(&nums[0..]);
    println!("largest num is {}", res);

    // trait 的使用
    let new = NewsArticle {
        headline: String::from("NA-headline"),
        location: String::from("NA-location"),
        author: String::from("NA-author"),
        content: String::from("NA-contetn"),
    };

    let tweet = Tweet {
        username: String::from("t-username"),
        content: String::from("t-content"),
        reply: false,
        retweet: false,
    };

    let summary = tweet.summarize();
    let summary2 = tweet.summarize2();
    println!("tweet summary is {}", summary);
    println!("tweet summary is {}", summary2);

    let weibo = build_weibo(); 
    println!("weibo summary is {}", weibo.summarize());
    println!("weibo summary2 is {}", weibo.summarize2());

    notify(new);
    notify(tweet);
    notify(weibo);
}

// 获取最大值
// fn largest(list: &[i32]) -> i32 {
//     let mut val = list[0];

//     for &i in list.iter() {
//         if i > val {
//             val = i;
//         }
//     }
//     val
// }

// largest 方法支持 泛型
fn largest<T>(list:&[T]) -> T where T : PartialOrd + Copy{
    let mut val = list[0];
    for &i in list.iter() {
        if i > val {
            val = i;
        }
    }

    val
}

// trait
pub trait Summary {
    // 不包含默认实现
    fn summarize(&self) -> String;

    // 提供了 summarize 的默认实现
    fn summarize2(&self) -> String {
        String::from("(Read More...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct WeiBo {
    pub username: String,
    pub content: String,
}

impl Summary for WeiBo {
    fn summarize(&self) -> String {
        format!("{}", self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("{}", item.summarize());
}

pub fn build_weibo() -> impl Summary {
    WeiBo {
        username: String::from("wb-username"),
        content: String::from("wb-content"),
    }
}
