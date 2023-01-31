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

    loop {
        let mut prompt = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        let prompt = prompt.trim();

        // let pat_random = Regex::new(r"^random$").unwrap();
        let pat_solving = Regex::new(r"^solve (\d+)$").unwrap();

        // let mut is_solving: bool = false;

        if pat_solving.is_match(prompt) {

            // is_solving = true;

            let id: u32 = pat_solving
                .captures(prompt)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            deal_solving(&id);
        } else {
            println!("Invalid command, please re-enter.");
        }
    }
}

fn deal_solving(id: &u32) { // -> Option<Problem> {

    // TODO: design to return an Error type and make the loop to continue
    let _problem = fetcher::get_problem(id).unwrap();
    println!("{:#?}", _problem);
}

