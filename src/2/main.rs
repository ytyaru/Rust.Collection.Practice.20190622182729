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
//    let employees = HashMap::new();
//    Department // Sales, PublicRelations, Developments

        println!("コマンド: quit, add 部署名 従業員名, list [部署名]");
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut s).ok();
        if !analize_command(&s.trim()) { break; }
    }
}
fn analize_command(s: &str) -> bool {
    match s {
        "quit" => false,
        _ => true,
    }
}
fn quit() {

}

