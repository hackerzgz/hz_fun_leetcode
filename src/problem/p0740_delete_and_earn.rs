pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }

    fn rob(nums: Vec<i32>) -> i32 {
        let mut f0 = 0;
        let mut f1 = 0;

        for n in nums {
            let new_f = f1.max(n + f0);
            f0 = f1;
            f1 = new_f;
        }

        f1
    }

    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap();
        let mut a = vec![0; mx as usize + 1];
        for x in nums {
            a[x as usize] += x; // 统计等于 x 的元素之和
        }
        Self::rob(a)
    }
}
