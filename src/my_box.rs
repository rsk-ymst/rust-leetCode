use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// impl Drop for MyBox<T> {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

mod tests {
    use super::MyBox;

    #[test]
    fn test1() {
        let x = 5;
        let y = MyBox::new(5);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
