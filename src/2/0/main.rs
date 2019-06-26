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

/*
    employees.insert(String::from("A"), Vec::new());
    println!("{:?}", employees);
    println!("{:?}", employees["A"]);

    // https://users.rust-lang.org/t/hashmap-with-vector-values/17906
    employees.get_mut("A").unwrap().push(String::from("A1"));
    println!("{:?}", employees);
    println!("{:?}", employees["A"]);
    employees.get_mut("A").unwrap().push(String::from("A2"));
    println!("{:?}", employees);
*/
    employees.entry("A".to_string()).or_insert(Vec::new());
    employees.get_mut("A").unwrap().push(String::from("A1"));
    employees.get_mut("A").unwrap().push(String::from("A2"));
//    employees.entry("A".to_string()).or_insert(String::from("A1"), Vec::new());
//    employees.entry("A".to_string()).or_insert(String::from("A2"), Vec::new());
    println!("{:?}", employees);
    employees.entry("B".to_string()).or_insert(Vec::new());
    employees.get_mut("B").unwrap().push(String::from("B1"));
//    employees.entry("B".to_string()).or_insert(String::from("B1"), Vec::new());
//    employees.get_mut("B").unwrap().push(String::from("B1"));
    println!("{:?}", employees);

    append(&mut employees, "C", "C1");
    append(&mut employees, "C", "C2");
    println!("{:?}", employees);

//    employees["A"].push(String::from("A1"));

    //println!("{:?}", employees[String::from("A")]);
    //println!("{:?}", employees["A".to_string()]);
    /*
    let emps = employees.entry("B".to_string()).or_insert(Vec::new());
    emps.push("B1".to_string());
    println!("{:?}", employees);
    */
}
fn append(employees: &mut HashMap<String, Vec<String>>, key: &str, value: &str) {
    employees.entry(key.to_string()).or_insert(Vec::new());
    employees.get_mut(key).unwrap().push(value.to_string());
}
