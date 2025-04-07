struct Solution {
    original: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { original: nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut nums = self.original.clone();
        for i in (0..nums.len()).rev() {
            let idx = rand::random_range(0..=i + 1);
            nums.swap(i, idx);
        }

        nums
    }
}
