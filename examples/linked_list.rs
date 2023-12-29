use std::borrow::Borrow;


#[derive(Debug)]

struct MyLinkedList {
    head: Option<Box<Node>>
}

#[derive(Debug)]

struct Node {
    val: i32,
    next: Option<Box<Node>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {

    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn get(&mut self, index: i32) -> i32 {
        // let old_head = self.head.unwrap().as_ref();
        // self.head = self.head
        0
    }

    fn add_at_head(&mut self, val: i32) {
        let old_head = self.head.take();

        let new_head = Node {
            val,
            next: old_head
        };

        self.head = Some(Box::new(new_head));

        return;
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_tail = Box::new(Node {
            val,
            next: None,
        });

        if self.head.is_none() {
            self.head = Some(new_tail);
            return;
        }

        let mut current = self.head.as_ref();
        let mut current = self.head.or_else(f);
        // self.head.and(optb)

        // Rustって人に例えると、勉強もスポーツも出来るハイスペックで
        // めっちゃ論理的・頭脳明晰なんだけど、
        // 真面目すぎて融通が利きにくいゆえに親しみづらい優等生、みたいなイメージ。
        // while let Some(cur_node) = self.head {
        //     if cur_node.next.is_none() {
        //         cur_node.next = Some(new_tail);
        //         return;
        //     }

        //     current = cur_node.next.as_mut();
        // }
    }

    fn add_at_index(&self, index: i32, val: i32) {

    }

    fn delete_at_index(&self, index: i32) {

    }
}

pub fn main() {

}

#[test]
pub fn test() {
    let mut x = MyLinkedList::new();

    x.add_at_head(1);
    x.add_at_head(2);

    println!("{:?}", x);
}

#[test]
pub fn test2() {
    let mut x = MyLinkedList::new();

    x.add_at_tail(1);
    x.add_at_tail(2);

    println!("{:?}", x);
}
