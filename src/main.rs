// 第三方依赖库：cargo add ferris-says
use ferris_says::say;

// 标准库：https://doc.rust-lang.org/std/index.html
use std::io::{self, stdout, BufWriter};

// 自定义模块
// 1、绝对路径以 crate 名称开头，相对路径使用self、super
// 2、文件名 src/xxx.rs 或 文件夹名 src/xxx/mod.rs
// use <crate>::<xxx>::<代码声明>
mod learn_base;
use crate::learn_base::test_base;

fn main() {
    // 基础语法
    test_base();

    // 命令行输入
    println!("请输入：");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    message.insert_str(0, "Hello,");
    let width = message.chars().count();

    // 命令行输出
    let mut writer = BufWriter::new(stdout().lock());
    say(&message, width, &mut writer).unwrap();
}
