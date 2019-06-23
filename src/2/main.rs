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
        let mut employees: HashMap<String, Vec<String>> = HashMap::new();
//    Department // Sales, PublicRelations, Developments

        println!("コマンド: quit, add 部署名 従業員名, list [部署名]");
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut s).ok();
        if !analize_command(&s.trim(), &mut employees) { break; }
    }
}
fn analize_command(s: &str, employees: &mut HashMap<String, Vec<String>>) -> bool {
    let commands: Vec<&str> = s.trim().split(' ').collect();
    match commands[0] {
        "quit" => false,
//        "add" => add_employee(commands[1..]),
//        "add" if 3 == commands.len() => add_employee(commands[1], commands[2]),
        "add" if 3 == commands.len() => {
//            let mut v = vec![String::from(commands[2])];
            let mut v = Vec::new();
            let emps = employees.entry(String::from(commands[1])).or_insert(v);
//            let emps = employees.entry(String::from(commands[1])).or_insert(Vec::new());
            emps.push(String::from(commands[2]));
//            let mut v = Vec::new();
//            v.push(String::from(commands[2]));

//            employees.insert(String::from(commands[1]), v);
//            println!("{:?}", employees);
//            println!("{:?}", *employees);
            println!("{:?}", *emps);
            true
        },
        "list" => {
            println!("{:?}", *employees);
            true
        },
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

