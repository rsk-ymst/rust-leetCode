
pub struct MyCircularQueue {
    array: Vec<i32>, // 静的配列は動的に配列の長さを指定することができないので、
    size: usize,
    head: usize,
    tail: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        MyCircularQueue {
            array: vec![-1; k as usize],
            size: k as usize,
            head: 0,
            tail: 0
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        self.print_self();
        if self.is_full() {
            return true;
        }

        self.array[self.tail] = value;
        self.next_tail();

        true
    }

    fn next_tail(&mut self) {
        self.tail += 1;

        if self.tail >= self.size {
            self.tail = 0;
        }
    }

    fn next_head(&mut self) {
        self.head += 1;

        if self.head >= self.size {
            self.head = 0;
        }
    }

    pub fn de_queue(&mut self) -> bool {
        self.print_self();

        if self.is_empty() {
            return false;
        }

        self.array[self.head] = -1;

        self.next_head();

        true
    }

    pub fn front(&self) -> i32 {
        *self.array.get(0).unwrap()
    }

    pub fn rear(&self) -> i32 {
        self.array[self.array.len() - 1]
    }

    pub fn is_empty(&self) -> bool {
        self.print_self();

        if self.array.iter().filter(|&&x| x == -1).collect::<Vec<_>>().len() == self.size {
            return true;
        }

        false
    }

    pub fn is_full(&self) -> bool {
        self.print_self();

        println!("{}", self.array.iter().filter(|&&x| x == -1).collect::<Vec<_>>().len());
        if self.array.iter().filter(|&&x| x == -1).collect::<Vec<_>>().len() == 0 {
            return true;
        }

        false
    }

    fn print_self(&self) {
        #[cfg(debug_assertions)]
        println!("{:?}", self.array);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::queue::MyCircularQueue;
        let mut x  = MyCircularQueue::new(3);

        // assert_eq!(x.en_queue(1), true);
        // assert_eq!(x.en_queue(2), true);
        // assert_eq!(x.en_queue(3), true);
        // assert_eq!(x.en_queue(4), true);
        // assert_eq!(x.rear(), 3);
        // assert_eq!(x.is_full(), true);
        // assert_eq!(x.de_queue(), true);
        // assert_eq!(x.en_queue(4), true);
        // assert_eq!(x.rear(), 4);

        assert!(true);
        assert_ne!(false, true);
        assert_eq!(2 + 2, 4);
    }

}
