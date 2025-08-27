use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4]),
            6
        );
        assert_eq!(Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]), 9);
        assert_eq!(Solution::zero_filled_subarray(vec![2, 10, 2019]), 0);
    }
}

impl Solution {
    // 等差数列
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut ans = 0_i64;

        let mut last = -1_i64;
        for (i, num) in nums.iter().enumerate() {
            if num != &0 {
                last = i as i64;
            } else {
                ans += i as i64 - last as i64;
            }
        }

        ans
    }
}
