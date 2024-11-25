struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut cursor: usize = 0;
        let mut ans = "".to_string();
        let mut command = command.into_bytes();

        loop {
            if command.get(cursor).is_none() {
                break;
            }

            let target = *command.get(cursor).unwrap();
            println!("cur: {}", target as char);

            if target == b'(' {
                cursor += 1;
                if let Some(&x) = command.get(cursor) {
                    match x {
                        b')' => {
                            ans.push('o');
                            cursor += 1;
                        }
                        b'a' => {
                            ans.push_str("al");
                            cursor += 2;
                        }
                        _ => {
                            panic!("err");
                        }
                    }
                }

                continue;
            }

            if target == b'G' {
                ans.push('G');
                cursor += 1;

                continue;
            }

            cursor += 1;
        }

        ans
    }
}

fn main() {
    let string = format!("G()()(al)");
    let x = Solution::interpret(string);
    println!("{x}");
}