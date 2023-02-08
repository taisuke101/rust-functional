pub fn string_contains() {
    let string = String::from("hello!");

    println!("string contains {:?}", string.contains("el"));
}

pub fn string_split() {
    let string = String::from("hello world!!");
    let split: Vec<&str> = string.split(' ').collect();

    println!("string split {:?}", split);
}
