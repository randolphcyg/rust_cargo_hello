use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: top, y: left },
        bottom_right: Point {
            x: bottom,
            y: right,
        },
    } = rect;

    ((top - bottom) * (right - left)).abs()
}

fn square(p: &Point, len: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: p.x,
            y: p.y + len,
        },
        bottom_right: Point {
            x: p.x + len,
            y: p.y,
        },
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[({}, {}), ()]\n[(), ({}, {})]",
            self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y
        )
    }
}

fn main() {
    let name = String::from("Ross");
    let age = 27;
    let ross = Person { name, age };

    // Debug方式打印结构体
    println!("{:?}", ross);

    // 实例化结构体Point
    let point: Point = Point { x: 10.3, y: 0.4 };
    // 访问point的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 point，
    // 这样可以用到之前的 point 的字段
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用 `let` 绑定来解构 point2
    let point2: Point = Point { x: 6.7, y: 0.1 };
    let Point {
        x: left_edge,
        y: top_edge,
    } = point2;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };
    println!("{}", _rectangle);

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect_area = rect_area(_rectangle);
    println!("rect_area is {:?}", rect_area);

    let point3: Point = Point { x: 6.4, y: 9.5 };
    let len: f32 = 5.0;
    let rect = square(&point3, len);
    println!("{}", rect);
}
