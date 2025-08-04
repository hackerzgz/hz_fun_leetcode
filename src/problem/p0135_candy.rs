pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }

    fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut left = vec![1; n];

        for i in 0..n {
            if i > 0 && ratings[i] > ratings[i - 1] {
                left[i] = left[i - 1] + 1;
            } else {
                left[i] = 1;
            }
        }

        let mut right = 0;
        let mut ret = 0;
        let mut i = n;
        while i > 0 {
            i -= 1;
            if i < n - 1 && ratings[i] > ratings[i + 1] {
                right += 1;
            } else {
                right = 1;
            }

            ret += left[i].max(right);
        }

        ret
    }
}
