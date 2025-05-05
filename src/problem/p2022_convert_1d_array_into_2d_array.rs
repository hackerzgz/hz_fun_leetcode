pub struct Solution;

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(&self) {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2),
            [[1, 2], [3, 4]]
        );
    }

    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() as i32 != m * n {
            return vec![];
        }

        original.chunks(n as usize).map(|x| x.to_vec()).collect()
    }
}
