pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Self {}
    }

    pub fn run(&self) {
        assert_eq!(
            Self::self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 23]
        );

        assert_eq!(Self::self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
    }

    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in left..right + 1 {
            if i % 10 == 0 {
                continue;
            }

            let mut temp = i;
            while temp > 0 {
                if temp % 10 == 0 || i % (temp % 10) != 0 {
                    break;
                }
                temp /= 10;
            }

            if temp == 0 {
                result.push(i as i32);
            }
        }
        result
    }
}
