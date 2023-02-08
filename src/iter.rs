pub fn map() {
    let v: Vec<i32> = vec![0, 1, 2, 3]
        .into_iter()
        // |x: i32| -> i32 {x + 1}のようにクロージャーの引数と返り値の型を明示することもできる
        .map(|x| x + 1)
        .collect();

    println!("map {:?}", v);
}

pub fn use_closure() {
    let add_one = |x| x + 1;
    let v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(add_one).collect();

    println!("use_closure {:?}", v);
}

pub fn get_index() {
    let v: Vec<i32> = vec![2, 3, 4, 5]
        .into_iter()
        .enumerate()
        .map(|(i, x): (usize, i32)| x + i as i32)
        .collect();

    println!("get_index {:?}", v);
}

pub fn filter() {
    let v = vec![0, 1, 2, 3];

    let filterd_v: Vec<i32> = v.into_iter().filter(|&x| x >= 2).collect();

    println!("filter {:?}", filterd_v)
}

pub fn sum() {
    let w = vec![4, 5, 6];

    let sum: i32 = w.into_iter().sum();

    println!("sum {:?}", sum);
}

pub fn fold() {
    let w = vec![5, 6, 7];

    let fold = w.into_iter().fold(0, |x, y| x * y);

    println!("fold {:?}", fold);
}

pub fn reduce() {
    let w = vec![4, 5, 6];

    let reduce = w.into_iter().reduce(|x, y| x + y).unwrap();

    println!("reduce {:?}", reduce);
}

pub fn currying(b: i32) -> impl Fn(i32) -> i32 {
    move |x| x + b
}

pub fn insert(x: i32, xs: &[i32]) -> Vec<i32> {
    match xs {
        [y, ys @ ..] => {
            if x <= *y {
                [&[x][..], xs].concat()
            } else {
                [&[*y][..], &insert(x, ys)].concat()
            }
        }
        [] => vec![x],
    }
}
