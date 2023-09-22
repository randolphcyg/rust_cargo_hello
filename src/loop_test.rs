#![allow(unreachable_code)]
#[allow(dead_code)]
pub fn main() {
    // 标签
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    // [从loop循环中返回]
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result:{}", result);
    assert_eq!(result, 20);
}
