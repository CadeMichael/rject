use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

fn main() {
    add_project("cade/world", &read_proj());
    add_project("test/world", &read_proj());
    add_project("test1/world", &read_proj());
    add_project("test2/world", &read_proj());

    let projs = read_proj();
    println!("{:?}", projs);
    remove_project("cade/world", projs);
    println!("{:?}", read_proj());
}

fn proj_path() -> PathBuf {
    // create path
    let home = match dirs::home_dir() {
        None => panic!("Cannot find home dir"),
        Some(h) => h,
    };
    // join $HOME and .proj
    let path = Path::new("").join(home).join(".proj");
    path.to_path_buf()
}

fn read_proj() -> Vec<String> {
    // open or create 
    let file = match File::open(proj_path()) {
        // proj file doesn't exist
        Err(_) => match File::create(proj_path()) {
            // can't create
            Err(e) => panic!("issue reading or creating file: {}", e),
            // empty file created (no need to read)
            Ok(_) => return vec![],
        },

        // return file
        Ok(file) => file,
    };

    // get buffer from file
    let buf = BufReader::new(file);
    // convert buffer to vector with no empty entries
    let projs: Vec<String> = buf
        .lines()
        .map(|l| l.expect("file cannot be read"))
        .filter(|p| !p.is_empty()) // no blank lines
        .collect();

    // return vector of projects
    projs
}

fn add_project(p: &str, projs: &[String]) {
    // make sure project isn't already present
    if projs.contains(&p.to_string()) {
        return
    }
    // open file for appending
    let mut file = OpenOptions::new()
        .append(true)
        .open(proj_path())
        .expect("cannot open proj file");

    // append new project
    file.write_fmt(format_args!("{}\n", p)).expect("wrote to file");
}

fn remove_project(proj: &str, projs: Vec<String>) -> Vec<String> {
    // open file (overwrite)
    let mut file = File::create(proj_path())
        .expect("cannot access proj file");

    // remove specified project
    let new_projs:Vec<String> = projs
        .into_iter()
        .filter(|p| p != proj)
        .collect();

    // re write file from scratch
    for p in new_projs.iter() {
        file.write_fmt(format_args!("{}\n", p))
            .expect("cannot write");
    }

    // return refactored list of projects
    new_projs
}
