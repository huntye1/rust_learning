mod apple;
mod pear;
mod orange;
use rand::Rng;

fn main() {
    let gen = rand::thread_rng().gen::<i64>() % 2;

    if gen == 0 {
        apple::eat_apple()
    }else {
       pear::eat_pear()
    }

    orange::eat::eat_orange();
    orange::foo::eat_foo();
}
