fn closure_base() {
    // 正常函数
    fn auto_add_fn(i: i32) -> i32 {
        i + 1
    }

    // 闭包：使用||括起参数、使用{}作为函数体界定符、有能力捕获外部变量
    let auto_add_clo = |i: i32| -> i32 { i + 1 };

    let i = 1;
    println!("auto_add_fn: {}", auto_add_fn(i));
    println!("auto_add_clo: {}", auto_add_clo(i));
}

fn closure_capture() {
    use std::mem;

    // [例子1]
    let color = String::from("green");
    // color会被一直保持被借用状态直到print离开作用域
    // println!只需传引用就能使用，而这个闭包也是变量的引用，因此无需进一步处理
    let print = || println!("`color`:{}", color);
    // 使用借用来调用闭包
    print();
    // color可再次被不可变借用，因为闭包只持有一个指向color的不可变引用
    let _reborrow = &color;
    print();
    // 再最后使用print后，移动或者重新借用都是允许的
    let _color_moved = color;

    // [例子2]
    let mut count = 0;
    // 该闭包立即借用count
    // int前需要加上mut，因为闭包里存储了一个&mut变量。
    // 调用闭包该变量变化意味着闭包内部变化，因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    // 使用可变借用调用闭包
    inc();
    // 重新借用会导致错误
    // let _reborrow = &count;
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // [例子3]
    let movable = Box::new(3);
    // drop要求T类型本身，所以该闭包会捕获变量的值；
    // 可复制类型复制给闭包，原始值不变。不可复制类型必须移动到表中。
    let consume = || {
        println!("`movable`：{:?}", movable);
        mem::drop(movable);
    };
    // consume消耗了该变量，所以只能调用一次
    consume();

    // [例子4]
    // 在||前使用move会强制闭包去的被捕获变量的所有权；
    // Vec不可复制
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //println!("There're {} elements in vec", haystack.len());
}

fn closure_input() {
    fn call_me<F: Fn()>(f: F) {
        f()
    }

    fn function() {
        println!("I'm a function!");
    }

    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
}

fn closure_output() {
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

#[allow(dead_code)]
pub fn main() {
    println!("{}", "***************");
    closure_base();
    println!("{}", "***************");
    closure_capture();
    println!("{}", "***************");
    closure_input();
    println!("{}", "***************");
    closure_output();
}
