pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) {
        assert_eq!(
            Solution::maximum_product(vec![-1, -9, 2, 3, -2, -3, 1], 1),
            81
        );
        assert_eq!(Solution::maximum_product(vec![1, 3, -5, 5, 6, -4], 3), 20);
    }

    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        let mut ans = i64::MIN;
        let mut mns = (i64::MAX, i64::MIN);

        let m = m as usize;
        for i in (m - 1)..nums.len() {
            let y = nums[i + 1 - m] as i64;

            mns.0 = mns.0.min(y);
            mns.1 = mns.1.max(y);

            let x = nums[i] as i64;
            ans = ans.max((x * mns.0).max(x * mns.1));
        }
        ans
    }
}
