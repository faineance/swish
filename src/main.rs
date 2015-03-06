use std::{io, os, env, path, process};

struct Job {
    process : process::Child,
    name : String
}
struct HistoryItem {
    args : Vec<String>
}

fn main() {
    let mut jobs: Vec<Job> = vec![];
    let mut history: Vec<HistoryItem> = vec![];

    loop {
        
    }
}