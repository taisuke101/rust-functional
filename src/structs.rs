#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

pub fn base_usage() {
    let taro = Person {
        name: String::from("taro"),
        age: 10,
    };

    println!("{:?}", taro);
}
