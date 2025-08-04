pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(self) {
        assert_eq!(Solution::can_make_equal(vec![1, -1, 1, -1, 1], 3), true);
        assert_eq!(
            Solution::can_make_equal(vec![-1, -1, -1, 1, 1, 1], 5),
            false
        );
    }

    fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        Solution::check(&nums, k, 1) || Solution::check(&nums, k, -1)
    }

    fn check(nums: &Vec<i32>, k: i32, target: i32) -> bool {
        let mut mul = 1;
        let mut remain = k;

        let num_len = nums.len();
        for i in 0..num_len {
            if nums[i] * mul == target {
                mul = 1;
                continue;
            }

            if remain == 0 || i == num_len - 1 {
                return false;
            }
            remain -= 1;
            mul = -1;
        }

        true
    }
}
