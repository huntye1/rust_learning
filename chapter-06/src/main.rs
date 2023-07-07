fn main() {
    let mut s = String::from("hello");

    // 多个不可变引用是允许的
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // 在这里，多个不可变引用是允许的，因此打印不会引发错误。

    // 一个可变引用
    let r3 = &mut s;
    println!("r3: {}", r3);
    // 在这里，只有一个可变引用，因此打印不会引发错误。

    // 不允许同时存在可变引用和不可变引用
    let r4 = &s; // 这会导致编译时错误
    // println!("r3: {}, r4: {}", r3, r4);
    println!("r4: {}", r4);
    // 如果取消注释上面两行，同时存在可变引用和不可变引用将导致编译时错误。
}