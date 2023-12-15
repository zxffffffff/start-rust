// 第三方依赖库：cargo add ferris-says
use ferris_says::say;

// 标准库：https://doc.rust-lang.org/std/index.html
use std::env;
use std::io::{self, stdout, BufWriter};

// mod 关键字声明模块
// 1、绝对路径以 crate 开头，相对路径使用 self、super
// 2、文件名 src/xxx.rs 或 文件夹名 src/xxx/mod.rs
// 3、支持别名 use crate::xxx::yyy as zzz
mod learn_base;
mod learn_base2;
mod learn_collection;
mod learn_struct;
mod learn_thread;
use crate::learn_base::test_base;
use crate::learn_base2::test_base2;
use crate::learn_collection::test_collection;
use crate::learn_struct::test_struct;
use crate::learn_thread::test_thread;

// main 也可以返回 Result<(), E>
fn main() {
    // 基础语法
    test_base();
    // 数据结构
    test_struct();
    // 容器
    test_collection();
    // 基础进阶
    test_base2();
    // 多线程
    test_thread();

    // 进程参数
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    // 命令行输入
    println!("输入你的名字：");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    if message.trim().is_empty() {
        message = String::from("World");
    }
    message.insert_str(0, "Hello, ");
    let width = message.chars().count();

    // 命令行输出
    let mut writer = BufWriter::new(stdout().lock());
    say(&message, width, &mut writer).unwrap();
}

// 单元测试
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
