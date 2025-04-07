pub struct Solution {}

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nmaps = std::collections::HashMap::with_capacity(2048);
        for n in nums {
            *nmaps.entry(n).or_insert(0) += 1;
        }

        let mut ans: i32 = 0;
        for (n, times) in nmaps.iter() {
            if let Some(t) = nmaps.get(&(n - 1)) {
                ans = ans.max(t + times);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_594() {
        assert_eq!(Solution::find_lhs(vec!(1, 3, 2, 2, 5, 2, 3, 7)), 5);
        assert_eq!(Solution::find_lhs(vec!(1, 2, 3, 4)), 2);
        assert_eq!(Solution::find_lhs(vec!(1, 1, 1, 1,)), 0);
    }
}
