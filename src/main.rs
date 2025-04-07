pub mod problem;

use clap::Parser;
use problem::{
    p0215_kth_largest_element_in_an_array, p0306_additive_number,
    p0318_maximum_product_of_word_lengths, p0319_bulb_switcher, p0384_shuffle_an_array,
    p0859_buddy_strings,
};

#[derive(Parser, Debug)]
struct Leetcode {
    #[arg(short, long)]
    problem: u16,
}

fn main() {
    let leetcode = Leetcode::parse();

    match leetcode.problem {
        215 => p0215_kth_largest_element_in_an_array::Solution::new().run(),
        306 => p0306_additive_number::Solution::new().run(),
        318 => p0318_maximum_product_of_word_lengths::Solution::new().run(),
        319 => p0319_bulb_switcher::Solution::new().run(),
        384 => p0384_shuffle_an_array::Solution::new().run(),
        859 => p0859_buddy_strings::Solution::new().run(),
        _ => unimplemented!("problem not found"),
    }
}
