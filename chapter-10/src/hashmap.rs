use std::collections::HashMap;

pub fn init() {
    let _: HashMap<String, i64> = HashMap::new();
    let _: HashMap<String, i64> = HashMap::with_capacity(16);
}

pub fn add() {
    // 类型自动推断
    let mut map = HashMap::new();
    map.insert("k".to_string(), "v".to_string());
}

pub fn get() {
    let mut map = HashMap::new();
    map.insert("k".to_string(), "v".to_string());

    println!("v is {:?}", map.get("k")); //some leix
}

pub fn modify_if() {
    let mut map = HashMap::new();
    map.insert("k1".to_string(), "v1".to_string());
    map.insert("k2".to_string(), "v2".to_string());

    println!("map is {:?}", map);

    map.entry("k1".to_string()).and_modify(|e| {
        if e == "v1" {
            *e = "v11".to_string();
        }
    });

    println!("map is {:?}", map);
}

pub fn remove() {
    let mut map = HashMap::new();
    map.insert("k1".to_string(), "v1".to_string());
    map.insert("k2".to_string(), "v2".to_string());
    println!("map is {:?}", map);

    map.remove("k1");
    println!("map is {:?}", map);
}


pub fn iter() {
    let mut map = HashMap::new();
    map.insert("k1".to_string(), "v1".to_string());
    map.insert("k2".to_string(), "v2".to_string());

    for (k, v) in &map {
        println!("{} : {}", k, v);
    }

    println!("map is {:?}", map);
}