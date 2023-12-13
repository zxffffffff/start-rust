// 第三方依赖库：cargo add ferris-says
use ferris_says::say;
// 标准库：https://doc.rust-lang.org/std/index.html
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
