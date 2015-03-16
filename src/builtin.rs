use std::{io, os, env, path, process};
pub fn cd(pathstr: &str) -> Result<(), ()> {
    let path = path::Path::new(&pathstr);
    env::set_current_dir(&path);
    Ok(())
    

}