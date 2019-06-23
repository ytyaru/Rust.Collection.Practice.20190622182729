/*
 * Rustのコレクション（練習問題）
 * CreatedAt: 2019-06-22
 */
fn main() {
    println!("{}", pig_latin("first"));
    println!("{}", pig_latin("apple"));
}
// https://doc.rust-jp.rs/book/second-edition/ch08-03-hash-maps.html#aハッシュマップと所有権
// * 各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。
// * ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)
// 
// Wikipediaとルールが違うのだが……上記を採用する。
//
// https://ja.wikipedia.org/wiki/ピッグ・ラテン
// 1. 子音で始まる単語は最初の母音の前の子音を語末へ移動し、その音節にay（発音は[ei]）を加える
// 2. [w]の音は子音と見なされ、他の子音と同じ扱いである
// 3. 母音で始まる単語（黙字の子音も含む）は語末にayを加える。
fn pig_latin(s: &str) -> String {
    let mut pig = String::from(s);

    let vowels = ['a','i','u','e','o']; // 母音
//    let consonants = ['b','c','d','f','g','h','j','k','l','m','n','p','q','r','s','t','v','w','x','y','z']; // 子音
//    match pig[0] { // error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`
//    match pig.get(0) { // error[E0277]: the type `str` cannot be indexed by `{integer}`
//    match pig.chars()[0] { // error[E0608]: cannot index into a value of type `std::str::Chars<'_>`
//    match pig.chars().nth(0).unwrap() {
    match s.chars().nth(0).unwrap() {
        'a'|'i'|'u'|'e'|'o' => pig.push_str("hay"),
//        _ => { pig.push(pig.get(0)); pig.remove(0); pig.push_str("ay"); },
        _ => { pig.push(s.chars().nth(0).unwrap()); pig.remove(0); pig.push_str("ay"); },
    }
    pig
}
