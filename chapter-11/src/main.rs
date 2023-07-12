mod  err;
// Option
struct User {
    id: u64,
    name: Option<String>,
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Result定义
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

#[derive(Debug)]
enum Version {
    Version1,
    Version2,
}

fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
    match header.get(3) {
        None => Err("invalid header length"),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("invalid version"),
    }
}

fn main() {
    // assert_eq!(divide(6.0, 3.0), Some(2.0));
    // assert_eq!(divide(6.0, 0.0), None);

    // let version = parse_version(&[1, 2, 3, 4]);
    // match version {
    //     Ok(v) => println!("working with version: {v:?}"),
    //     Err(e) => println!("error parsing header: {e:?}"),
    // }

    err::run()
}
