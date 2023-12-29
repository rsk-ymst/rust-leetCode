use std::cell::Cell;

pub struct inter {
    a: Cell<usize>,
    b: usize,
}


// RefCell: コンパイラのメモリ安全検証をかいぐぐる手段。
// 所有権の移動にはmoveって言葉が使われるよ～
pub fn test() {
    let x = inter { a: Cell::new(0), b: 0};
    x.a.set(10);
    // x.b += 1;
    // y.get();
}
