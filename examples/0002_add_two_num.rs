use std::borrow::Borrow;


struct Solution {}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let l1_vec: Vec<i32> = Self::get_vec(&l1, &l2);

        let sum = Self::get_sum(&l1_vec) + Self::get_sum(&l2_vec);

        Some(Box::new(ListNode { val: 0, next: None }))
    }

    // 文字列にして、結合
    pub fn get_sum(vec1: &Vec<i32>) -> i32 {
        let x = vec1.iter().map(|x| format!("{x}") ).collect::<Vec<String>>();
        let y = x.join("").parse::<i32>().unwrap();

        y
    }

    pub fn get_vec(node1: &Option<Box<ListNode>>, node2: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut buf1 = node1.clone();
        let mut buf2 = node2.clone();

        let mut ret_vec: Vec<i32> = vec![];
        let mut is_up = false;

        loop {
            match (buf1.take(), buf2.take()) {
                (Some(n1), Some(n2)) => {
                    let sum = n1.val + n2.val + if is_up {1} else {0};

                    is_up = sum >= 10;
                    let digit = (sum) % 10;

                    ret_vec.push(digit);

                    buf1 = n1.next;
                    buf2 = n2.next;
                },
                (None, Some(n2)) => {
                    let sum = n2.val + if is_up {1} else {0};

                    is_up = sum >= 10;
                    let digit = (sum) % 10;

                    buf2 = n2.next;
                    ret_vec.push(digit);
                },
                (Some(n1), None) => {
                    let sum = n1.val + if is_up {1} else {0};

                    is_up = sum >= 10;
                    let digit = (sum) % 10;

                    buf1 = n1.next;
                    ret_vec.push(digit);
                },
                (None, None) => {
                    ret_vec.push(if is_up {1} else {0});
                    break;
                },
            }
        }

        ret_vec
    }
}

pub fn main() {

}
