//値を返さない関数の場合 ->を省略可能
pub fn collect() {
    let c = "あいうえお";

    //Vec<_>で推論させるのも可
    let c_vec: Vec<char> = c.chars().collect();

    println!("c_vec {:?}", c_vec);
}
