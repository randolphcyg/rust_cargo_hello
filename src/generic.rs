struct A;

struct SingleGen<T>(T);

pub fn run() {
    let _s = SingleGen(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A); // 使用在上面定义的 `A`。
    let _i32 = SingleGen(6); // 使用 `i32` 类型。
    let _char = SingleGen('a'); // 使用 `char`。
}
