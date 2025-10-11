use std::collections::BTreeMap;

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::maximum_total_damage(vec![1, 1, 3, 4]), 6);
    }
}

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut magics: BTreeMap<i32, i32> = BTreeMap::new();
        for p in power {
            *magics.entry(p).or_insert(0) += 1;
        }

        let mut stack = Vec::with_capacity(magics.len());
        for (pow, count) in magics {
            stack.push((pow, count));
        }

        let mut ans = 0_i64;
        let mut dp = vec![0 as i64; stack.len() + 1];
        for i in 0..stack.len() {}

        ans
    }
}
