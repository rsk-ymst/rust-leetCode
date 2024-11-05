
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut iter = s.chars().peekable();

        while let Some(cur_symbol) = iter.next() {
            let plus_point = match cur_symbol {
                'I' => match iter.peek() {
                        Some('V') => {
                            iter.next();
                            4
                        },
                        Some('X') => {
                            iter.next();
                            9
                        },
                        _ => 1
                    },
                'V' => 5,
                'X' => match iter.peek() {
                        Some('L') => {
                            iter.next();
                            40
                        },
                        Some('C') => {
                            iter.next();
                            90 
                        },
                        _ => 10
                    },
                'L' => 50,
                'C' => {
                    match iter.peek() {
                        Some('D') => {
                            iter.next();
                            400
                        },
                        Some('M') => {
                            iter.next();
                            900
                        },
                        _ => 100
                    } 
                },
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };
            sum += plus_point;
        }

        sum 
    }
}

fn main() {}