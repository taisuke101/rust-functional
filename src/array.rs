pub fn len() {
    println!("{}", vec![5, 6].len());
}

pub fn count() {
    println!("{}", vec![5, 6].iter().count());
}

pub fn contains() {
    let v = vec![5, 6];
    println!("{}", v.contains(&6));
}

pub fn get() {
    let v = vec![5, 6];
    println!("{}", v.get(1).unwrap());
    println!("{:?}", v.get(..2).unwrap());
    println!("{}", v[1]);
}
