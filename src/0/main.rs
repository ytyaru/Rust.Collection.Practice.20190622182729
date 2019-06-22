/*
 * Rustのコレクション（練習問題）
 * CreatedAt: 2019-06-22
 */
fn main() {
    let v = vec![5, 2, 1];
    println!("vec: {:?}", v);
    println!("mean: {}", mean(&v));
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
    v.sort(); // error[E0596]: cannot borrow immutable borrowed content `*v` as mutable
    0
}
// 頻出値
fn mode(v: &Vec<i32>) -> i32 {
    0
}
