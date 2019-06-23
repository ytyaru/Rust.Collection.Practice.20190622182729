/*
 * Rustのコレクション（練習問題）
 * CreatedAt: 2019-06-22
 */
fn main() {
    let v = vec![5, 2, 7, 1, 3, 2, 9];
    println!("vec: {:?}", v);
    println!("mean: {}", mean(&v));
    println!("median: {}", median(&v));
    println!("mode: {}", mode(&v));
}
// 平均
fn mean(v: &Vec<i32>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in v {
        sum += f64::from(*i); // https://stackoverflow.com/questions/55080089/converting-i32-to-f64
//        let f = i as f64; // error[E0606]: casting `&i32` as `f64` is invalid
//        sum += f;
//        sum += i as f64;
//        sum += i; // error[E0277]: cannot add-assign `&i32` to `f64`
    }
    sum / v.len() as f64
//    sum / v.len() // error[E0277]: cannot divide `f64` by `usize`
    // https://qiita.com/nacika_ins/items/7017f926b7eb4f8cec8a
}
// ソート時の中央値
fn median(v: &Vec<i32>) -> i32 {
//    v.sort(); // error[E0596]: cannot borrow immutable borrowed content `*v` as mutable
//    let mut s = v; // error[E0596]: cannot borrow immutable borrowed content `*s` as mutable
//    s.sort();
    let mut s = Vec::new();
    for i in v { s.push(i); }
    s.sort();
    **s.get(s.len()/2).unwrap()
}
// 頻出値
fn mode(v: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    for i in v {
        let count = h.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (key, value) in h {
//        println!("{} {}", key, value);
        if max_count < value { max_count = value; mode = *key; }
    }
    mode
}

