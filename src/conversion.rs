use std::convert::{TryFrom, TryInto};
// use std::fmt;
use std::string::ToString;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32,
}

// impl fmt::Display for Circle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Circle of radius {}", self.radius)
//     }
// }

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

#[allow(dead_code)]
pub fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("{}", my_string);

    // [from] 将i32类型转换为自定义类型Number
    let num = Number::from(30);
    println!("my num is {:?}, value is {}", num, num.value);

    // [into] trait是from的倒过来，只要类型实现了from。则可以免费获得into
    let int = 5;
    // 自定义类型into trait需要指明类型，编译器无法推断
    let num: Number = int.into();
    println!("my number is {:?}", num);

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    // println!("{:?}", result);
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    // println!("{:?}", result);
    assert_eq!(result, Err(()));

    // [to_string] 实现Display后会自动提供
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // [to_string]
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // 解析字符串
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println! {"Sum: {:?}", sum};
}
