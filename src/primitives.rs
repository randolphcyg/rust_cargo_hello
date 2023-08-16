fn main() {
    let logical: bool = true;
    println!("{}", logical);

    let a_float: f64 = 1.0;
    let an_inter = 5i32;

    // let default_float = 7;
    // ley mut inferred_type = 12;
    // inferred_type = 4294967296i64;

    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;

    // 这里使用u32会溢出： attempt to compute `1_u32 - 2_u32`, which would overflow
    println!("1 + 2 = {}", 1i32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    // 短路求值的布尔逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);
}
