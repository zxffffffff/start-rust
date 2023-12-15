// use std::fmt::Debug;
// use std::fmt::Display;
// use std::fs::File;
// use std::io::ErrorKind;

pub fn test_base2() {
    error_handling();
    generic_types();
    traits();
}

fn error_handling() {
    // 错误分为两大类：可恢复错误(recoverable)、不可恢复错误(unrecoverable)

    // panic!("crash!!!"); // 崩溃，和越界等错误一样

    // 使用 Result
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // 使用 unwrap/expect 等价为 match Err panic!
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("hello.txt").expect("more msg");

    // 使用闭包和 unwrap_or_else
    // let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // 使用 ? 运算符（返回值为 Result、Option）
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    let _c = last_char_of_first_line("abc");
    let _c2 = last_char_of_first_line("");
}

fn generic_types() {
    let p = Point { x: 5, y: 10 };
    println!("p = ({},{})", p.x(), p.y);
}

// 泛型
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn traits() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
}

// 特征 trait，类似于 interface/override
pub trait Summary {
    fn summarize(&self) -> String;
    // 默认实现
    fn default_foo(&self) -> String {
        String::from("default_foo")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 在 struct 上实现
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 传参
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 泛型搭配特征
// fn some_function<T, U>(t: &T, u: &U)
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }
