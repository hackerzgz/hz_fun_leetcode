use std::usize;

pub struct Solution;

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        assert!(Self::is_additive_number("112358"))
    }

    fn is_additive_number(num: &str) -> bool {
        (0..num.len() - 1).any(|i| valid_from(i, &num))
    }
}

fn valid_from(second_start: usize, num: &str) -> bool {
    if second_start > 1 && num.chars().nth(0).unwrap() == '0' {
        return false;
    }
    (second_start..num.len() - 1).any(|i| valid(second_start, i, &num))
}

fn valid(second_start: usize, second_end: usize, num: &str) -> bool {
    if second_end > second_start && num.chars().nth(second_start).unwrap() == '0' {
        return false;
    }

    let first: i64 = num[0..second_start].parse().unwrap();
    let second: i64 = num[second_start..second_end + 1].parse().unwrap();
    second_end + 1 < num.len() && is_can_added(first, second, num, second_end + 1)
}

fn is_can_added(first: i64, second: i64, num: &str, sum_idx: usize) -> bool {
    if sum_idx == num.len() {
        return true;
    }

    let sum_str = i64::to_string(&(first + second));
    if sum_idx + sum_str.len() > num.len() {
        return false;
    }

    let actual_sum = &num[sum_idx..sum_idx + sum_str.len()];
    actual_sum == sum_str && is_can_added(second, first + second, num, sum_idx + sum_str.len())
}
