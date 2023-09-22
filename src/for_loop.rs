#[allow(dead_code)]
pub fn main() {
    for i in 1..3 {
        println!("左闭右开[):{}", i)
    }

    for i in 1..=3 {
        println!("双端包含[]:{}", i)
    }

    println!("{}", "***************");

    let names = vec!["Ross", "Chandler", "Joey"];

    for name in names.iter() {
        match name {
            &"Chandler" => println!("There is a sweety among us! [{}]", name),
            _ => println!("Hello! {}", name),
        }
    }
    println!("names: {:?}", names);

    println!("{}", "***************");
    // into_iter会消耗集合
    let names2 = vec!["Ross", "Chandler", "Joey"];
    for name2 in names2.into_iter() {
        match name2 {
            "Chandler" => println!("There is a sweety among us! [{}]", name2),
            _ => println!("Hello! {}", name2),
        }
    }

    println!("{}", "***************");
    // 就地修改集合
    let mut names3 = vec!["Ross", "Chandler", "Joey"];
    for name3 in names3.iter_mut() {
        *name3 = match name3 {
            &mut "Chandler" => "There is a sweety among us!",
            _ => "Hello!",
        }
    }
    println!("names3: {:?}", names3);
}
