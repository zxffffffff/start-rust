use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn test_thread() {
    threads();
    thread_move();
    thread_channel();
    thread_mutex();
}

fn threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn thread_move() {
    let v = vec![1, 2, 3];

    // 使用 move 关键字强制闭包获取其使用的值的所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn thread_channel() {
    // 类似 Go 使用通道通讯
    // mpsc 代表 多个生产者，单个消费者
    let (provider, consumer) = mpsc::channel();

    {
        // 可以克隆
        let _provider2 = provider.clone();
    }

    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            provider.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    let handle2 = thread::spawn(move || {
        // 等同于 let val = consumer.recv().unwrap();
        // 当通道关闭时，迭代将结束
        for val in consumer {
            println!("Got: {}", val);
        }
    });

    handle.join().unwrap();
    handle2.join().unwrap();
}

fn thread_mutex() {
    // 互斥锁(由智能指针实现)，存在死锁的问题
    // 原子操作(new/clone)，存在性能代价
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Arc<Mutex<i32>> counter = {}", *counter.lock().unwrap());
}
