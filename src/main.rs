use std::{io, os, env, path, process};
use std::io::BufRead;
use std::io::Write;

use builtin::{cd};
mod builtin;

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
        let mut input = String::new();
        let outputbuffer = io::stdout();
        let mut outputlock = outputbuffer.lock();
        let prompt = prompt();                                                                                                                                                                                                           
        outputlock.write(prompt.as_bytes());
        outputlock.flush();
        
        
        let inputbuffer = io::stdin();
        let mut inputlock = inputbuffer.lock();
        inputlock.read_line(&mut input);
        


    }
}
