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
    prompt.push_str("$ ");
    prompt
}
fn execute(args: Vec<&str>) -> Result<(), ()> {
    match args[0] {
        "exit" => {Err(())},
        "cd" => {

            builtin::cd(args[1]) 
        }
        _ => {
            let mut command = process::Command::new(args[0]);
            command.args(args.tail().as_slice());
            match command.spawn(){
                Ok(_) => (),
                Err(e) => println!("Invalid command: {}", e)

            }
            Ok(())

        }
    }
}


    fn main() {
        let mut jobs: Vec<Job> = vec![];
        let mut history: Vec<HistoryItem> = vec![];
        let mut locals: Vec<(String, String)> = Vec::new();
        println!("Swish");
        loop {

            let outputbuffer = io::stdout();
            let mut outputlock = outputbuffer.lock();
            let prompt = prompt();                                                                                                                                                                                                           
            outputlock.write(prompt.as_bytes());
            outputlock.flush();

            let mut input = String::new();
            let inputbuffer = io::stdin();
            let mut inputlock = inputbuffer.lock();
            inputlock.read_line(&mut input);

            if input.trim() == "" {continue}
            let mut args: Vec<&str> = input.trim().words().collect();
            match execute(args) {
                Ok(_) => (),
                Err(e) => {break}
            }
            

        }
    }
