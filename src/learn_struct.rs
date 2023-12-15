pub fn test_struct() {
    defining_structs();
    defining_enum();
    match_control_flow();
}

fn defining_structs() {
    let _unit_like = UnitLikeStruct;

    let mut user1 = User {
        active: false,
        username: String::from("zxf"),
        email: String::from("zxffffffff@outlook.com"),
        sign_in_count: 0,
    };
    user1.active = true;
    user1.sign_in_count = 1;

    let user2 = build_user(user1.email, user1.username);
    println!("user2 = {:#?}", user2); // use `{:?}` (or {:#?} for pretty-print)

    // let user3 = user1; // 编译报错，email 和 username 已经移动
    let mut user3 = dbg!(user2); // 返回对象，并输出到 stderr

    // 调用方法
    user3.foo();
    User::bar();
}

// 没有字段的结构体
struct UnitLikeStruct;

// #[derive(Debug)] 用于支持 Debug/print
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // 可以简写同名字段
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

impl User {
    // 成员函数 this
    fn foo(&mut self) {
        self.active = true;
    }

    // 静态成员函数 static
    fn bar() {}
}

fn defining_enum() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let _ = home.kind;
    let _ = loopback.address;

    let _home2 = IpAddr2::V4(127, 0, 0, 1);
    let _loopback2 = IpAddr2::V6(String::from("::1"));

    let _quit = Message::Quit;
    let _move = Message::Move { x: 50, y: 100 };
    // let _x = _move.x;
    // let _y = _move.y;
    let _write = Message::Write(String::from("msg"));
    let _color = Message::ChangeColor(255, 0, 0);

    // 标准库内置的泛型枚举
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None; // 比 null 更好
}

// 枚举
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 枚举支持定义类型
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 各种类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn match_control_flow() {
    // match 类似 switch
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let alabama = Coin::Quarter(UsState::Alabama);
    let alaska = Coin::Quarter(UsState::Alaska);
    let _value = value_in_cents(penny);
    let _value = value_in_cents(nickel);
    let _value = value_in_cents(dime);
    let _value = value_in_cents(alabama);
    let _value = value_in_cents(alaska);

    // match Some 等价于 if let Some
    let config_max = Some(100);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        ()
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
