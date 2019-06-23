/*
 * Rustのコレクション（練習問題）
 * CreatedAt: 2019-06-22
 */
use std::collections::HashMap;
use std::io::Write;
/*
 * add 部署名 従業員名
 * list [部署名]
 * q
 */
fn main() {
    loop {
        let mut s = String::new();
        let mut employees: HashMap<String, &mut Vec<String>> = HashMap::new();
//    Department // Sales, PublicRelations, Developments

        println!("コマンド: quit, add 部署名 従業員名, list [部署名]");
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut s).ok();
        if !analize_command(&s.trim(), &mut employees) { break; }
    }
}
fn analize_command(s: &str, employees: &mut HashMap<String, &mut Vec<String>>) -> bool {
    let commands: Vec<&str> = s.trim().split(' ').collect();
    match commands[0] {
        "quit" => false,
//        "add" => add_employee(commands[1..]),
//        "add" if 3 == commands.len() => add_employee(commands[1], commands[2]),
        "add" if 3 == commands.len() => {
//            let emps = employees.entry(commands[1]).or_insert(Vec::new()); // error[E0308]: mismatched types
//            let emps = employees.entry(String::from(commands[1])).or_insert(Vec::new()); // error[E0308]: mismatched types
//            let emps = employees.entry(String::from(commands[1])).or_insert(&mut Vec::new()); // error[E0308]: mismatched types
//            let emps = employees.entry(String::from(commands[1])).or_insert(&mut vec![String::from(commands[2])])); // error[E0308]: mismatched types
            let mut v = Vec::new();
            let emps = employees.entry(String::from(commands[1])).or_insert(&mut v); // error[E0308]: mismatched types
//            *emps.get(String::from(commands[1])).push(commands[2]) // error[E0277]: the type `[std::string::String]` cannot be indexed by `std::string::String`
//            *emps.push(String::from(commands[2])); // error[E0614]: type `()` cannot be dereferenced
//            emps.push(String::from(commands[2])); // error[E0597]: borrowed value does not live long enough
            true
        }
        _ => true,
    }
    /*
    match s {
        "quit" => false,
        _ => true,
    }
    */
}
fn add_employee(department: &str, name: &str) {
    
}
fn quit() {

}

