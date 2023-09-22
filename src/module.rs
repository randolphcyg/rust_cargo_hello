use uuid::Uuid;

use deeply::nested::function as other_function;

mod my {
    use uuid::Uuid;

    pub struct OpenBox<T> {
        pub uuid: Uuid,
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        uuid: Uuid,
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                uuid: Uuid::new_v4(),
                contents,
            }
        }
    }
}

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    // [结构体可见性]
    let id = Uuid::new_v4();
    println!("The open box id: {}", id);

    let open_box = my::OpenBox {
        uuid: Uuid::new_v4(),
        contents: "public information",
    };

    println!(
        "The open box uuid: {}, contents: {}",
        open_box.uuid, open_box.contents
    );

    // let closed_box = my::ClosedBox { contents: "classified information" };

    let _closed_box = my::ClosedBox::new("classified information");
    // println!("The _closed_box uuid: {}, contents: {}", _closed_box.uuid, _closed_box.contents);

    // [use 模块别名]
    other_function();

    {
        println!("Entering block");
        use deeply::nested::function;
        function();
        println!("Leaving block");
    }

    function();
}
