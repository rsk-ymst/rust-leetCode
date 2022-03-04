fn main() {
    println!("Hello, world!");

    let nums = vec![0, 1, 2, 3, 4];

    for e in &nums {
        println!("e -> {}, *e -> {}", e, *e);
    }
}
