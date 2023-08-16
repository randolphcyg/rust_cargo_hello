mod array;
mod display;
mod enumeration;
mod format;
mod linked_list_enum;
mod list;
mod primitives;
mod structure;
mod tuple;
mod variable;

fn main() {
    println!("Hello rust World");
    println!("I'm a Rustacean!");

    let x = 5 + /* 90+ */ 5;
    println!("5 +5 is {}", x);

    // 使用{}作为占位符
    println!("a month contains {} days mostly", 31);
    // 从0开始标记占位符是第几个参数
    println!(
        "{0}, this is {1}, {2}, {3} and {4}!",
        "July", "Monica", "Phoebe", "Chandler", "Joey"
    );
    // 命名参数
    println!(
        "{sister}'s brother is {brother}",
        sister = "Monica",
        brother = "Ross"
    );

    // 在:后指定字符格式化数字为不同进制
    println!("base 10:{}", 344560);
    println!("base 2:{:b}", 344560);
    println!("base 8:{:o}", 344560);
    println!("base 16:{:x}", 344560);
    println!("base 16:{:X}", 344560);

    // 右对齐指定宽度，总长度5，4个空格和1 "    1"
    println!("{number:>5}", number = 1);
    // 右对齐 用0补充 "00001"
    println!("{number:0>5}", number = 1);
    // 左对齐 用0补充 "10000"
    println!("{number:0<5}", number = 1);

    // 使用$在格式说明中使用命名参数
    println!("{number:0>width$}", number = 1, width = 5);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    println!("Pi is roughly {pi:.*}", 3, pi = 3.141592);

    // 小数点后五位 .N ,N$ .*
    println!("Hello {0} is {1:.5}", "x", 0.01);
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);
    println!("Hello {} is {:.*}", "x", 5, 0.01);

    // 调试 不加debug traits 不能使用std::fmt打印
    // struct UnPrintable(i32);
    // println!("Now {:?} will not print!", UnPrintable(233));

    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("Now {:?} will print!", DebugPrintable(233));
    println!("Now {:?} will print!", Deep(DebugPrintable(233)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Ross";
    let age = 27;
    let ross = Person { name, age };

    println!("{:#?}", ross)
}
