mod queue;

use core::num;
use std::{array, borrow::BorrowMut};
// use sort;

fn main() {
    // println!("Hello, world!");
    //
    // let mut nums = vec![0, 1, 2, 3, 4];
    //
    // for e in &nums {
    //     println!("e -> {}, *e -> {}", e, *e);
    // }

    let mut x  = queue::MyCircularQueue::new(3);

    x.en_queue(1);
}

// struct Solution {
//     hoge: i32,
// }
//
// impl Solution {
//     pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
//         let mut res = Vec::new();
//
//         for mut e in nums {
//             let buf = e.clone();
//             res.push(buf.pow(2));
//         }
//
//         res.sort();
//         return res;
//     }
//
//
//     /*
//     nums が mutableなものとして宣言されていれば、以下の様に可変参照でfor文をまわせる
//     for e in &mut nums {
//         *e = (*e).pow(2)
//     }
//     */
//
//     pub fn duplicate_zeros(arr: &mut Vec<i32>) {
//         let len = arr.len();
//         let mut pre_zero = false;
//
//         for i in 0..len {
//             if pre_zero {
//                 pre_zero = false;
//                 continue;
//             }
//
//             // println!("{}", arr[i]);
//             if arr[i] == 0 {
//                 // println!("{}", arr[i]);
//                 arr.insert(i+1, 0);
//                 pre_zero = true;
//             }
//         }
//         arr.split_off(len);
//     }
//
//     // 最後にsplit_offをするのではなく、insert/deleteを同時に行うのも有用
//
//     pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
//         // let len = if m == 0 {0} else {nums1.len()};
//         // let start = len - (m as usize);
//         let start_idx = m as usize;
//         for i in 0..(n as usize) {
//             nums1[start_idx+1] = nums2[i];
//         }
//         nums1.sort();
//     }
//
//     // impl Solution {
//     //     pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
//     //         nums1.split_off(m as usize);
//     //         nums1.extend(nums2.to_vec());
//     //         nums1.sort();
//     //     }
//     // }
//
//     pub fn hoge() -> i32 {
//         for i in (0..10) {
//
//         }
//         0
//     }
//     // # Remove Element
//     // nums のイテレータの中でnumsをいじっては行けない。
//     // pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     //     let mut mut_nums = nums;
//
//     //     for (i, &x) in nums.iter().enumerate() {
//     //         if x == val {
//     //             mut_nums = nums.remove(i);
//     //         }
//     //     }
//
//     //     0
//     // }
//
//     pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//         let mut rm_count = 0;
//         let mut idx = 0;
//
//         for i in (0..total).rev() {
//
//         }
//
//         loop {
//             if idx >= nums.len() {
//                 break;
//             }
//
//             // nums.retain(f);
//
//             if val == nums[idx] {
//                 nums.swa(idx);
//
//                 rm_count += 1;
//                 idx -=1;
//             }
//
//             idx += 1;
//         }
//
//         rm_count
//     }
//
//     // # Check If N and Its Double Exist
//     pub fn check_if_exist(arr: Vec<i32>) -> bool {
//         let length = arr.len();
//
//
//         for i in 0..length {
//             for j in 0..length {
//                 let con1 = i != j;
//                 let con2 = 0 <= i && j < length;
//                 let con3 = arr[i] == 2 * arr[j];
//
//                 if con1 && con2 && con3 {
//                     return true;
//                 }
//             }
//         }
//
//         false
//     }
//
//
//     pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
//         let mut arr: &mut Vec<i32> = arr.as_mut();
//         let mut prev_max = -1;
//
//         for i in (0..arr.len()).rev() {
//             arr[i] = prev_max;
//
//             if prev_max < arr[i] {
//                 prev_max = arr[i];
//             }
//         }
//
//         arr.to_vec()
//     }
//
//     pub fn replace_elements(mut A: Vec<i32>) -> Vec<i32> {
//         let mut max = -1;
//
//         for x in A.iter_mut() {
//             let t = *x;
//             *x = max;
//             max = std::cmp::max(max, t);
//         }
//         A
//     }
//
//
//     // xxxxxxxxxxx
//     pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
//         let mut edge_count = 0;
//         let mut prev = 100000;
//
//         for i in 0..arr.len() - 1 {
//             if arr[i] > prev && arr[i] > arr[i+1] {
//                 edge_count += 1;
//             }
//             prev = arr[i];
//         }
//
//         if edge_count == 1 {
//             return true;
//         } else {
//             return false;
//         }
//     }
//
//
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         let mut prev = 1000;
//
//         for i in (0..nums.len()).rev() {
//             if prev == nums[i] {
//                 nums.remove(i);
//             }
//             prev = nums[i]
//         }
//
//         nums.len() as i32
//     }
//
//     pub fn move_zeroes(nums: &mut Vec<i32>) {
//         for i in (0..nums.len()).rev() {
//             if nums[i] == 0 {
//                 nums.remove(0);
//                 nums.push(0);
//             }
//         }
//     }
// }
