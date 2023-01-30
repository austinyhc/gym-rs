#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod fetcher;

use std::io;
use std::fs;
use std::fs::File;
use regex::Regex;

fn main() {
    println!("Welcome to gym-rs.\n");
    println!(
		"Please enter a frontend problem id, \n\
			or \"random\" to generate a random one, \n\
			or \"solve $i\" to move problem to solutions/, \n\
			or \"all\" to initialize all problems \n");


    // let mut id_arg = String::new();
    // io::stdin()
    //     .read_line(&mut id_arg)
    //     .expect("Failed to read line");
    let id_arg = "solve 919";

    let id_arg = id_arg.trim();

    let pat_random = Regex::new(r"^random$").unwrap();
    let pat_solving = Regex::new(r"^solve (\d+)$").unwrap();

    let mut is_solving: bool = false;
    let mut id: u32 = 0;

    if pat_solving.is_match(id_arg) {

        is_solving = true;

        id = pat_solving
            .captures(id_arg)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        println!("You select {id}");
        deal_solving(&id);
    }
}

fn deal_solving(id: &u32) { // -> Option<Problem> {
    println!("{id}");
    let problem = fetcher::get_problem(id);//.unwrap();
}

