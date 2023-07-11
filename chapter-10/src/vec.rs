pub fn init_vec() {
    // 初始化长度为 0
    let _: Vec<i32> = Vec::new();

    // 初始化长度为 10
    let _: Vec<i32> = Vec::with_capacity(10);

    // 宏初始化
    let _ = vec!["hello", "world"];

    // 类型推断
    let mut v = Vec::new();
    v.push(12);

    // 直接指定类型
    let _ = Vec::<String>::new();
}

pub fn get_by_index() {
    let v = vec!["1", "2"];

    // 性能会差一点
    let v1 = v.get(1);
    let v2 = v.get(2);

    println!("v1 is {:?}", v1);
    println!("v2 is {:?}", v2); //None

    v[1];
    // 直接 panic
    v[2];
}

pub fn modify_by_index() {
    let mut v = vec!["1", "1"];
    v[1] = "2";

    // 直接 panic
    // v[2] = "100";

    v.push("3");

    println!("v is {:?}", v);
    v.remove(1);

    println!("v is {:?}", v);

    v.push("3");
    v.push("4");
    v.push("5");

    // 把最后一个元素，移到被删除元素的位置
    println!("v is {:?}", v);
    v.swap_remove(1);
    println!("v is {:?}", v);
}

pub fn inter() {
    let mut v = vec!["1", "2"];

    // for in 使用IntoIterator trait
    for el in &v {
        println!("el is {:?}", el);
    }

    println!("v is {:?}", v);

    for el in &mut v {
        *el = "foo"
    }

    println!("v is {:?}", v);
}
