use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        let mut obj = NumArray::new(vec![4, 2, 5]);
        obj.update(1, 1);
        let ret_2: i32 = obj.sum_range(0, 2);
        assert_eq!(ret_2, 10);
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
struct NumArray {
    seq_nodes: Vec<i32>,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let num_len = nums.len();
        let mut num_array = NumArray {
            seq_nodes: vec![0; 4 * nums.len() + 7],
            nums,
        };

        num_array.build(1, 0, num_len);
        num_array
    }

    fn build(&mut self, p: usize, l: usize, r: usize) {
        if l == r {
            self.seq_nodes[p] = self.nums[l];
            return;
        }

        let mid = (l + r) >> 1;
        self.build(p << 1, l, mid);
        self.build(p << 1 | 1, mid + 1, r);
        self.seq_nodes[p] = self.seq_nodes[p << 1] + self.seq_nodes[p << 1 | 1];
    }

    fn change(&mut self, index: usize, val: i32, p: usize, l: usize, r: usize) {
        if l == r {
            self.seq_nodes[p] = val;
            return;
        }

        let mid = (l + r) >> 1;
        if index <= mid {
            self.change(index, val, p << 1, l, mid);
        } else {
            self.change(index, val, p << 1 | 1, mid + 1, r);
        }
        self.seq_nodes[p] = self.seq_nodes[p << 1] + self.seq_nodes[p << 1 | 1];
    }

    fn range_(
        &self,
        left: usize,
        right: usize,
        p: usize,
        curr_left: usize,
        curr_right: usize,
    ) -> i32 {
        if curr_left > right || curr_right < left {
            return 0;
        }

        if curr_left >= left && curr_right <= right {
            return self.seq_nodes[p];
        }

        let mid = (curr_left + curr_right) >> 1;
        self.range_(left, right, p << 1, curr_left, mid)
            + self.range_(left, right, p << 1 | 1, mid + 1, curr_right)
    }

    fn update(&mut self, index: i32, val: i32) {
        self.change(index as usize, val, 1, 0, self.nums.len() - 1);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.range_(left as usize, right as usize, 1, 0, self.nums.len() - 1)
    }
}
