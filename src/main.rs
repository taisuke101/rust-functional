mod array;
mod collect;
mod iter;
mod read_json;
mod string;
mod structs;

use array::{contains, count, get, len};
use collect::collect;
use iter::{currying, filter, fold, get_index, insert, map, reduce, sum, use_closure};
use read_json::{read_json, User};
use string::{string_contains, string_split};
use structs::base_usage;

fn main() {
    println!("Hello, world!");

    collect();
    map();
    use_closure();
    get_index();
    filter();
    sum();
    fold();
    reduce();

    let add_2 = currying(2);
    println!("add_2 {}", add_2(1));

    let v: Vec<i32> = Vec::new();
    let v = insert(2, &v);
    let v = insert(8, &v);
    let v = insert(5, &v);
    println!("insert {:?}", v);

    contains();
    count();
    get();
    len();

    let users: Vec<User> = read_json("sample.json").unwrap();

    println!("{:?}", users);

    string_contains();
    string_split();

    base_usage();
}
