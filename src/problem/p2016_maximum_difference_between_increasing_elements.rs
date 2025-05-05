pub struct Solution;

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(&self) {
        assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4]), 4);
        assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
        assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
    }

    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((-1, i32::MAX), |(mut ans, mut cur_min), num| {
                if cur_min < *num {
                    ans = ans.max(*num - cur_min);
                }
                cur_min = cur_min.min(*num);
                (ans, cur_min)
            })
            .0
    }
}
