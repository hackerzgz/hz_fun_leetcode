use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }
    pub fn run(&self) {}

    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(n) = root {
                let mut n = n.borrow_mut();
                let left_sum = dfs(n.left.take());
                let right_sum = dfs(n.right.take());
                (
                    left_sum.0 + right_sum.0 + (left_sum.1 - right_sum.1).abs(),
                    left_sum.1 + right_sum.1 + n.val,
                )
            } else {
                (0, 0)
            }
        }
        dfs(root).0
    }
}
