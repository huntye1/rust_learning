struct User {
    username: String,
    email: String,
    age: u8,
}

fn main() {
    let user1 = User {
        username: String::from("Alice"),
        email: String::from("alice@example.com"),
        age: 18,
    };
    
    // let user2 = User {
    //     username: user1.username,
    //     email: String::from("alice2@example.com"),
    //     age: user1.age,
    // };

    let user2 = User {
        email: String::from("alice2@example.com"),
        ..user1
    };

    println!("user2 is {}", user2)


}