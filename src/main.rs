use std::{io, os, env, path, process};


struct Job {
    process : process::Child,
    name : String
}
struct HistoryItem {
    args : Vec<String>
}
fn prompt() -> String {
    let mut prompt = String::new();
    let pwd = Path::new(os::getenv("PWD").unwrap());
    let homedir = Path::new(os::homedir().unwrap());

    if pwd == homedir {
        prompt.push_str("~")
    } else {
        prompt.push_str(pwd.as_str().unwrap());
    }
    prompt.push_str("$");
    prompt

    

}


fn main() {
    let mut jobs: Vec<Job> = vec![];
    let mut history: Vec<HistoryItem> = vec![];
    println!("Swish");
    loop {
        let prompt = prompt();

    }
}