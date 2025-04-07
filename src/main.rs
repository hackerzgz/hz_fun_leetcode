pub mod problem;

use clap::Parser;
use leetcode_rs::problem;

#[derive(Parser, Debug)]
struct Leetcode {
    #[arg(short, long)]
    problem: u16,
}

fn main() {
    let leetcode = Leetcode::parse();

    match leetcode.problem {
        859 => problem::p859_buddy_strings,
    }
}
