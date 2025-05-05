pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Self {}
    }

    pub fn run(&self) {
        assert_eq!(
            Solution::push_dominoes(String::from(".L.R...LR..L.")),
            "LL.RR.LLRRLL."
        );
        assert_eq!(Solution::push_dominoes(String::from("RR.L")), "RR.L");
    }

    pub fn push_dominoes(dominoes: String) -> String {
        // let mut result = String::from(dominoes.as_str());
        let mut dominoes = format!("L{}R", dominoes).into_bytes();

        // 1. find a non status card
        // 2. got left and right card within status
        // 3. match the status pattern
        //   - L.R => L.R (only the middle card still in standing)
        //   - R.L => R.L (only the middle card still in standing)
        //   - L.L => LLL
        //   - R.R => RRR

        let mut i = 0;
        for j in 1..dominoes.len() {
            if dominoes[j] != b'.' {
                match (dominoes[i], dominoes[j]) {
                    (b'L', b'L') => dominoes[i..j].iter_mut().for_each(|x| *x = b'L'),
                    (b'R', b'R') => dominoes[i..j].iter_mut().for_each(|x| *x = b'R'),
                    (b'R', b'L') => {
                        for k in 1..(j - i + 1) / 2 {
                            dominoes[i + k] = b'R';
                            dominoes[j - k] = b'L';
                        }
                    }
                    _ => (),
                }

                i = j;
            }
        }

        dominoes.remove(0);
        dominoes.pop();

        String::from_utf8(dominoes).unwrap()
    }
}
