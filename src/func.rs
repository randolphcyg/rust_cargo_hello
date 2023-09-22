fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    // 表达式可以不用return关键字
    lhs % rhs == 0
}

// 一个 “不” 返回值的函数。实际上会返回一个单元类型 `()`。
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回 `()` 时，函数签名可以省略返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

// 高阶函数
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn higher_order_fn() {
    let upper = 1000;
    let mut acc = 0;

    for n in 0.. {
        let n_squard = n * n;
        if n_squard >= upper {
            break;
        } else if is_odd(n_squard) {
            acc += n_squard;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

// [发散函数] 不会返回，后面的代码逻辑不会执行
// 可以用在构造特殊逻辑，模拟永远无法达成的条件
fn foo() -> ! {
    panic!("发散函数永不返回！");
}

// 其他正常函数会返回
fn some_fn() {
    ()
}

fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        // 注意这个 match 表达式的返回值必须为 u32，
        // 因为 “addition” 变量是这个类型。
        let addition: u32 = match i % 2 == 1 {
            true => i,
            // 另一方面，“continue” 表达式不返回 u32，但它仍然没有问题，
            // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
            false => continue,
        };
        acc += addition;
    }
    acc
}

#[allow(dead_code)]
// 和 C/C++ 不一样，Rust 的函数定义位置是没有限制的
pub fn main() {
    // 我们可以在这里使用函数，后面再定义它
    fizzbuzz_to(100);

    higher_order_fn();

    // foo();
    let _a: () = some_fn();

    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}
