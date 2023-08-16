use std::fmt;
use std::fmt::Formatter;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 使用元祖下标获取值，创建一个vec的引用
        let vec = &self.0;
        write!(f, "[")?;

        // 使用v对vec迭代，用count记录迭代次数
        for (count, v) in vec.iter().enumerate() {
            // 使用？返回错误
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        // 末尾中括号，并返回值
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v)
}
