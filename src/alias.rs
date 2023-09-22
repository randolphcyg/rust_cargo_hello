// `NanoSecond` 是 `u64` 的新名字。 类型的名字必须遵循驼峰命名法
type NanoSecond = u64;
type Inch = u64;

// 屏蔽警告。
#[allow(non_camel_case_types)]
type u64_t = u64;

#[allow(dead_code)]
pub fn main() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
