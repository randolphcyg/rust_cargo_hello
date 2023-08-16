fn main() {
    let age = 18;
    println!("Tell me about {}", age);

    // 只会走一个分支
    match age {
        // 匹配单个值
        1 => println!("One!"),
        //  匹配多个值
        2 | 3 | 5 | 7 | 11 | 18 => println!("This is a prime"),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        20..=65 => println!("A postadolescent"),
        66..=100 => println!("A elderly person"),
        // 处理其他情况 没有会报错
        _ => println!("Ain't special"),
    }
}
