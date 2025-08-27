use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true);
        assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false);
        assert_eq!(Solution::judge_point24(vec![8, 1, 6, 6]), true);
    }
}

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let s = Solution::new();

        let cards = cards.into_iter().map(|x| x as f64).collect::<Vec<f64>>();
        s.dfs(&cards)
    }

    pub fn dfs(&self, cards: &Vec<f64>) -> bool {
        if cards.len() == 0 {
            return false;
        }

        if cards.len() == 1 {
            return (cards[0] as f64 - 24.0).abs() < 1e-6;
        }

        for i in 0..cards.len() {
            for j in i + 1..cards.len() {
                for op in 0..6 {
                    let next_list = self.get_next_list(cards, i, j, op);
                    if !next_list.is_empty() && self.dfs(&next_list) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_next_list(&self, cards: &Vec<f64>, i: usize, j: usize, op: usize) -> Vec<f64> {
        let mut new_cards = Vec::with_capacity(cards.len() - 2);

        match op {
            0 => new_cards.push(cards[i] + cards[j]),
            1 => new_cards.push(cards[i] - cards[j]),
            2 => new_cards.push(cards[j] - cards[i]),
            3 => new_cards.push(cards[i] * cards[j]),
            4 => {
                if cards[j].abs() < 1e-6 {
                    return vec![];
                }
                new_cards.push(cards[i] / cards[j]);
            }
            5 => {
                if cards[i].abs() < 1e-6 {
                    return vec![];
                }
                new_cards.push(cards[j] / cards[i]);
            }
            _ => unreachable!(),
        }

        for k in 0..cards.len() {
            if k != i && k != j {
                new_cards.push(cards[k]);
            }
        }

        println!("new card: {:?}", new_cards);
        new_cards
    }
}
