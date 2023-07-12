use std::panic;
pub fn run() {
    let r = panic::catch_unwind(|| {
        panic!("Panic in catch_unwind()...");
    });

    assert!(r.is_err());

    let r = panic::catch_unwind(|| {
        println!("No panic...");
    });

    assert!(r.is_ok());

    let mut variable = 4;

    // This code will not compile because the closure captures &mut variable
    // which is not considered unwind safe by default.
    // panic::catch_unwind(|| {
    //     variable += 3;
    // });
}
