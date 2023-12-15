// 默认是 private，需要添加 public
pub fn test_base() {
    variables_and_mutability();
    data_types();
    functions(123, 123.456);
    control_flow();
    ownership();
    references_and_borrowing();
    slices();
}

fn variables_and_mutability() {
    let _a = 123; // 只读变量
    let mut _b = 123; // 变量
    const _C: i32 = 123; // 常量，必须注释类型，大写

    // _a = 456; // 只读
    let _a = "456"; // 新的变量可重名，可不同类型

    _b = 456;
    // _b = "456"; // 变量赋值不能改变类型
    _b = "456".parse().expect("Not a number!");

    // const _C: i32 = 456; // 新的常量不可重名
}

fn data_types() {
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // 元组：固定长度，支持不同类型
    let tup = (500, 6.4, 1);
    // 解构元组
    let (_x, _y, _z) = tup;
    // 索引访问
    let _a = tup.0;
    let _b = tup.1;
    let _c = tup.2;

    // 数组：固定长度，相同类型
    let _arr = [1, 2, 3, 4, 5];
    let _arr2 = [6; 3]; // [6, 6, 6]
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // 索引访问
    let _first = months[0];
    let _second = months[1];

    // let _out_of_bounds = months[12]; // 编译时报错
    // 编译器无法识别，不会提示！
    let _i: usize = "12".parse().expect("");
    // let _out_of_bounds = months[_i]; // 越界，程序崩溃
}

fn functions(input: i32, input2: f64) -> (i32, f64) {
    // 返回值不带分号;
    let ret = (input * 100, input2 * 100.0);
    ret
}

fn control_flow() {
    // if 表达式
    let condition = true;
    let number = if condition { 1 } else { -1 };
    if number < 0 {
        // false
    } else if number > 0 {
        // true
    } else {
        // false
    }

    // loop 返回值
    let mut counter = 0;
    let _result = loop {
        counter += 1;
        if counter == 5 {
            continue;
        }
        if counter == 10 {
            break counter * 2;
        }
    };

    // loop 嵌套：需要以单引号'开头
    'flag: loop {
        loop {
            // break;
            break 'flag;
        }
    }

    // while 条件循环
    let mut i = 10;
    while i > 0 {
        i -= 1;
        println!("i = {i}");
    }

    // for 循环遍历
    let a = ['a', 'b', 'c'];
    for element in a {
        println!("element = {element}");
    }

    // for range [start, end)
    for number in (0..10).rev() {
        // rev 反转
        println!("number = {number}");
    }
}

fn ownership() {
    // 类似 RAII 模式，离开作用域时释放内存
    {
        let _s = "hello";
    } // Drop _s

    // 堆数据：不是深拷贝也不是浅拷贝，而是 move
    let s1 = String::from("hello");
    let s2 = s1;
    // let s3 = s1; // 编译报错，已经移动
    let _s4 = s2.clone(); // 手动clone堆数据

    // 栈数据：复制基本数据类型 bool char i32 f64
    let n1 = 1;
    let _n2 = n1;
    let _n3 = n1;
    let _n4 = n1.clone(); // 可以省略clone

    // 函数传参也 move
    fn takes_ownership(s: String) {
        let _s = s;
    }
    let param = String::from("xxx");
    takes_ownership(param);
    // let _s = param; // 编译报错，已经移动
}

fn references_and_borrowing() {
    // & 引用/指针，没有所有权
    fn references(s: &String, s2: &mut String) {
        let _s = s;
        s2.push_str(s)
    }
    let s = String::from("hello");
    let mut s2 = s.clone();
    references(&s, &mut s2);
    let _s = s;

    // 悬空引用/指针
    // fn dangle() -> &String { // 编译报错
    //     let s = String::from("hello");
    //     &s
    // }
    // let reference_to_nothing = dangle();
}

fn slices() {
    // 切片
    let s = String::from("hello world");
    let _hello = &s[..5]; // [0..5]
    let _world = &s[6..]; // [6..11]
    let _total = &s[..]; // [0..11]
}
