use std::i32;

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3), 3);
        assert_eq!(Solution::maximum_energy(vec![8, -5], 1), 3);
    }
}

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let k_usize = k as usize;
        let mut ans = i32::MIN;

        for i in (n - k_usize)..n {
            let mut curr = 0;
            let mut j = i as i32;
            while j >= 0 {
                curr += energy[j as usize];
                ans = ans.max(curr);
                j -= k_usize as i32;
            }
        }

        ans
    }
}
