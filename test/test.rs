#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut x  = queue::MyCircularQueue::new(3);

        x.en_queue(1);
        assert_eq!(x.en_queue(1), true);

        x.en_queue(2);
        x.en_queue(3);
        x.en_queue(4);
        x.rear();
        x.is_full();

        assert_eq!(2 + 2, 4);
    }
}
