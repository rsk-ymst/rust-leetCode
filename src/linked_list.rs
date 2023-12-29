#[derive(Clone, Debug)]
struct MyLinkedList {
    value: i32,
    next : Option<Box<MyLinkedList>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { value: 0, next: None }
    }

    fn get(&self, index: i32) -> i32 {
        let mut buf: MyLinkedList = self.clone();

        for i in 0..index {
            println!("{:?}", buf);

            match buf.next {
                Some(x) => {
                    buf = *x
                },
                None => return buf.value,
            }
        }

        buf.value
    }

    fn add_at_head(&mut self, val: i32) {
        let mut new_one = MyLinkedList { value: val, next: None };
        new_one.next = Some(Box::new(self.clone()));

        *self = new_one;
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut new_one = MyLinkedList { value: val, next: None };

        if let Some(ref mut x) = self.next {}

        loop {
            println!("{:?}", buf);

            match buf.next {
                Some(x) => buf = *x,
                None => break,
            }
        }

        buf.next = Some(Box::new(new_one));
    }

    fn add_at_index(&self, index: i32, val: i32) {}

    fn delete_at_index(&self, index: i32) {}
}

pub fn test() {
    let mut hoge = MyLinkedList::new();

    hoge.add_at_head(0);
    hoge.add_at_head(1);
    hoge.add_at_head(2);
    hoge.get(2);

    println!("{:#?}", hoge);
    println!("{:#?}", hoge.get(0));
}
