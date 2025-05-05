pub mod problem;

use clap::Parser;
use problem::{
    p0215_kth_largest_element_in_an_array, p0306_additive_number,
    p0318_maximum_product_of_word_lengths, p0319_bulb_switcher, p0384_shuffle_an_array,
    p0397_integer_replacement, p0400_nth_digit, p0423_reconstruct_original_digits_from_english,
    p0563_binary_tree_tilt, p0594_longest_harmonious_subsequence, p0677_map_sum_pairs,
    p0728_self_dividing_numbers, p0827_making_a_large_island, p0838_push_dominoes,
    p0859_buddy_strings, p1025_divisor_game, p1185_day_of_the_week,
    p2016_maximum_difference_between_increasing_elements, p2022_convert_1d_array_into_2d_array,
};

#[derive(Parser, Debug)]
struct Leetcode {
    #[arg(short, long)]
    problem: u16,
}

macro_rules! problem {
    ($prob:ident) => {
        $prob::Solution::new().run()
    };
}

fn main() {
    let leetcode = Leetcode::parse();

    match leetcode.problem {
        215 => problem!(p0215_kth_largest_element_in_an_array),
        306 => problem!(p0306_additive_number),
        318 => problem!(p0318_maximum_product_of_word_lengths),
        319 => problem!(p0319_bulb_switcher),
        384 => problem!(p0384_shuffle_an_array),
        397 => problem!(p0397_integer_replacement),
        400 => problem!(p0400_nth_digit),
        423 => problem!(p0423_reconstruct_original_digits_from_english),
        563 => problem!(p0563_binary_tree_tilt),
        594 => problem!(p0594_longest_harmonious_subsequence),
        677 => problem!(p0677_map_sum_pairs),
        728 => problem!(p0728_self_dividing_numbers),
        827 => problem!(p0827_making_a_large_island),
        838 => problem!(p0838_push_dominoes),
        859 => problem!(p0859_buddy_strings),
        1025 => problem!(p1025_divisor_game),
        1185 => problem!(p1185_day_of_the_week),
        2016 => problem!(p2016_maximum_difference_between_increasing_elements),
        2022 => problem!(p2022_convert_1d_array_into_2d_array),
        _ => unimplemented!("problem not found"),
    }
}
