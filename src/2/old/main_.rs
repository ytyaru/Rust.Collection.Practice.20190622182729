/*
 * Rustのコレクション（練習問題）
 * CreatedAt: 2019-06-22
 * 
 * HashMap<_,mut Vec<String>>`のようにコレクションのネストができない……
 */
use std::collections::HashMap;
use std::io::Write;
/*
 * add 部署名 従業員名
 * list [部署名]
 * q
 */
fn main() {
//    let mut employees: HashMap<String, &mut Vec<String>> = HashMap::new();
//    let mut employees: HashMap<String, mut Vec<String>> = HashMap::new(); // error: expected one of `>`, const, lifetime, or type, found `mut`
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    employees.insert(String::from("A"), Vec::new());
    println!("{:?}", employees);
    println!("{:?}", employees["A"]);

    // https://users.rust-lang.org/t/hashmap-with-vector-values/17906
    employees.get_mut("A").unwrap().push(String::from("A1"));
    println!("{:?}", employees);
    println!("{:?}", employees["A"]);

//    employees["A"].push(String::from("A1"));

    //println!("{:?}", employees[String::from("A")]);
    //println!("{:?}", employees["A".to_string()]);
    /*
    let emps = employees.entry("B".to_string()).or_insert(Vec::new());
    emps.push("B1".to_string());
    println!("{:?}", employees);
    */
}

