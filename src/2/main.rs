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
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut s = String::new();

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
        "add" if 2 < commands.len() => {
            add_employee(employees, commands[1], commands[2]);
            true
        },
        "list" if 1 == commands.len() => { println!("{:?}", employees); true },
        "list" if 1 < commands.len() => { println!("{:?}", employees.get(commands[1])); true },
        _ => true,
    }
}
fn add_employee(employees: &mut HashMap<String, Vec<String>>, department: &str, employee: &str) {
    employees.entry(department.to_string()).or_insert(Vec::new());
    employees.get_mut(department).unwrap().push(employee.to_string());
}

