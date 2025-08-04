pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(self) {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}
