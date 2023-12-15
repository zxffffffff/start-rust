pub fn test_collection() {
    vectors();
    strings();
    hash_maps();
}

fn vectors() {
    let _v: Vec<i32> = Vec::new();

    // vec! 宏
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    let first = &v[0];
    // v.push(6); // 编译报错，push 会导致内存地址变化，和引用发生冲突
    let mut v = vec![100, 32, 57];
    dbg!(first);
    for i in &mut v {
        *i += 50; // 编译成功，+= 不会导致内存地址变化，仅修改内存值
    }

    let _third: &i32 = &v[2];
    let _third: Option<&i32> = v.get(2);

    // let does_not_exist = &v[100]; // 越界，程序崩溃
    let does_not_exist = v.get(100); // 返回 None
    match does_not_exist {
        Some(_n) => (),
        None => println!("v.get(100) => None"),
    }
}

fn strings() {
    // 字符串是 UTF-8 编码
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // 区分 String 和 &str
    let _s = String::new();
    let s1 = String::from("aaa");
    let s2 = "bbb";
    let s3 = "ccc".to_string();

    let s = format!("{s1}-{s2}-{s3}");
    println!("{} {} {} {}", s, s1, s2, s3);
}

fn hash_maps() {
    // 默认情况下， HashMap 使用名为 SipHash 的哈希函数，该函数可以抵抗涉及哈希表 1 的拒绝服务 (DoS) 攻击。
    // 这不是可用的最快的哈希算法，但为了获得更好的安全性而带来的性能下降是值得的。
    // 如果您分析代码并发现默认哈希函数对于您的目的而言太慢，则可以通过指定不同的哈希器来切换到另一个函数。
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 5);
    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Blue"), 15); // 覆盖
    scores.entry(String::from("Yellow")).or_insert(20); // 不存在时插入

    let team_name = String::from("Blue");
    // get 返回 Option<&i32>
    // copied 拷贝引用
    // unwrap_or 替换 None 为默认值
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
