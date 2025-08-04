use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(self) {
        assert_eq!(Solution::special_triplets(vec![6, 3, 6]), 1);
        assert_eq!(Solution::special_triplets(vec![0, 1, 0, 0]), 1);
        assert_eq!(Solution::special_triplets(vec![8, 4, 2, 8, 4]), 2);
    }

    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut suff: HashMap<i32, i32> = HashMap::new();
        for &n in nums.iter() {
            *suff.entry(n).or_insert(0) += 1;
        }

        let mut ans = 0_isize;
        let mut pre: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *suff.entry(num).or_insert(0) -= 1;

            let k = num * 2;
            ans += (*pre.get(&k).unwrap_or(&0) as isize) * (*suff.get(&k).unwrap_or(&0)) as isize;
            *pre.entry(num).or_insert(0) += 1;
        }

        (ans % 1_000_000_007) as i32
    }
}
