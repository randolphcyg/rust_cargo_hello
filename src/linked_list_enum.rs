use List::*;

enum List {
    // 元祖结构体：包含一个元素和指向下一节点的指针
    Cons(u32, Box<List>),
    // 末结点
    Nil,
}

impl List {
    // 创建一个空的List实例
    fn new() -> List {
        Nil
    }

    // 处理一个List，在头部插入新元素，并返回该List
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回列表长度
    fn len(&self) -> u32 {
        match *self {
            // 不能得到tail的所有权，因为self是借用的
            // 因此使用一个对tail的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 递归的基准情形：一个长度为0的空列表
            Nil => 0,
        }
    }

    // 返回列表字符的字符串表示(该字符串是堆分配的)
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
