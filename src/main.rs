pub mod problem;
pub mod week;

use clap::Parser;
use problem::*;
use week::*;

#[derive(Parser, Debug)]
struct Leetcode {
    #[arg(short, long)]
    problem: Option<u16>,

    #[arg(short, long)]
    week: Option<String>,
}

macro_rules! problem {
    ($prob:ident) => {
        $prob::Solution::new().run()
    };
}

fn main() {
    let leetcode = Leetcode::parse();

    if let Some(pb) = leetcode.problem {
        match pb {
            135 => problem!(p0135_candy),
            215 => problem!(p0215_kth_largest_element_in_an_array),
            306 => problem!(p0306_additive_number),
            307 => problem!(p0307_range_sum_query_mutable),
            318 => problem!(p0318_maximum_product_of_word_lengths),
            319 => problem!(p0319_bulb_switcher),
            384 => problem!(p0384_shuffle_an_array),
            397 => problem!(p0397_integer_replacement),
            400 => problem!(p0400_nth_digit),
            423 => problem!(p0423_reconstruct_original_digits_from_english),
            // 559 => problem!(p0559_maximum_depth), // TODO:
            560 => problem!(p0560_subarray_sum),
            563 => problem!(p0563_binary_tree_tilt),
            594 => problem!(p0594_longest_harmonious_subsequence),
            677 => problem!(p0677_map_sum_pairs),
            679 => problem!(p0679_judge_point24),
            728 => problem!(p0728_self_dividing_numbers),
            740 => problem!(p0740_delete_and_earn),
            808 => problem!(p0808_soup_servings),
            827 => problem!(p0827_making_a_large_island),
            838 => problem!(p0838_push_dominoes),
            859 => problem!(p0859_buddy_strings),
            869 => problem!(p0869_reordered_power_of_2),
            904 => problem!(p0904_total_fruit),
            1025 => problem!(p1025_divisor_game),
            1185 => problem!(p1185_day_of_the_week),
            1277 => problem!(p1277_count_squares),
            1733 => problem!(p1733_minimum_teachings),
            1780 => problem!(p1780_check_powers_of_three),
            2016 => problem!(p2016_maximum_difference_between_increasing_elements),
            2106 => problem!(p2106_max_total_fruits),
            2022 => problem!(p2022_convert_1d_array_into_2d_array),
            2131 => problem!(p2131_longest_palindrome),
            2327 => problem!(p2327_people_aware_of_secret),
            2348 => problem!(p2348_zero_filled_subarray),
            2438 => problem!(p2438_range_product_queries_of_powers),
            2561 => problem!(p2561_rearranging_fruits),
            2785 => problem!(p2785_sort_vowels),
            2787 => problem!(p2787_ways_to_express_an_integer_as_sum_of_powers),
            3227 => problem!(p3227_does_alice_win),
            3363 => problem!(p3363_maximum_number_of_fruits_collected),
            3477 => problem!(p3477_num_of_unplaced_fruits),
            3478 => problem!(p3478_num_of_unplaced_fruits),
            3560 => problem!(p3560_min_cutting_cost),
            3576 => problem!(p3576_can_make_equal),
            3583 => problem!(p3583_special_triplets),
            3584 => problem!(p3584_maximum_product),

            _ => unimplemented!("problem not found"),
        }
    }

    if let Some(w) = leetcode.week {
        match w.as_str() {
            "463q1" => problem!(w0463q1_max_profit),

            _ => unimplemented!("week not found"),
        }
    }
}
