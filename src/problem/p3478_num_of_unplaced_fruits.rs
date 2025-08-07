use std::i32;

use crate::problem::Solver;

pub struct Solution;

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]),
            1
        );

        assert_eq!(
            Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]),
            0
        );
    }
}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let basket_len = baskets.len();
        if basket_len == 0 {
            return fruits.len() as i32;
        }
        let mut ans = 0_i32;
        let mut tree = SegTree::new(baskets);

        for f in fruits {
            let (mut l, mut r, mut res) = (0_usize, basket_len, None);
            while l <= r {
                let mid = (l + r) / 2;

                if tree.query(1, 0, basket_len - 1, 0, mid) >= f {
                    res = Some(mid);
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }

            if let Some(pos) = res {
                if tree.baskets[pos] >= f {
                    tree.update(1, 0, basket_len - 1, pos, i32::MIN);
                    continue;
                }
            }
            ans += 1;
        }
        ans
    }
}

struct SegTree {
    seg_node: Vec<i32>,
    baskets: Vec<i32>,
}

impl SegTree {
    fn new(baskets: Vec<i32>) -> Self {
        let n = baskets.len();
        let mut tree = SegTree {
            seg_node: vec![0; 4 * n + 7],
            baskets,
        };
        tree.build(1, 0, n - 1);
        tree
    }

    // p 是线段树节点的索引下标，特别地，它由 1 开始
    // p 是层次遍历满二叉树的当前节点索引下标
    //
    // l / r 是 baskets 数组的索引下标
    fn build(&mut self, p: usize, l: usize, r: usize) {
        if l == r {
            self.seg_node[p] = self.baskets[l];
            return;
        }

        let mid = (l + r) / 2;
        // p * 2 是线段树数组的左子节点下标
        self.build(p * 2, l, mid);
        // p * 2 + 1 是线段树数组的右子节点下标
        self.build(p * 2 + 1, mid + 1, r);
        self.seg_node[p] = self.seg_node[p * 2].max(self.seg_node[p * 2 + 1]);
    }

    fn query(&mut self, p: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
        if ql > r || qr < l {
            return i32::MIN;
        }

        if ql <= l && r <= qr {
            return self.seg_node[p];
        }

        let mid = (l + r) / 2;
        self.query(p * 2, l, mid, ql, qr)
            .max(self.query(p * 2 + 1, mid + 1, r, ql, qr))
    }

    fn update(&mut self, p: usize, l: usize, r: usize, pos: usize, val: i32) {
        if l == r {
            self.seg_node[p] = val;
            return;
        }

        let mid = (l + r) / 2;
        if pos <= mid {
            self.update(p * 2, l, mid, pos, val);
        } else {
            self.update(p * 2 + 1, mid + 1, r, pos, val);
        }

        self.seg_node[p] = self.seg_node[p * 2].max(self.seg_node[p * 2 + 1]);
    }
}
