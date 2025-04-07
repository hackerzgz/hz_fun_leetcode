struct Solution;
impl Solution {
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
