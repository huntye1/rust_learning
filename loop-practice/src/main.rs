fn main() {
    let target = 10;
    let mut sum = 0;
    let mut counter = 1;

    let result = loop {
        sum += counter;

        if sum >= target {
            break counter; // The value of counter will be returned from the loop as a result
        }

        counter += 1;
    };

    println!(
        "The first number whose sum of all previous numbers is greater than or equal to {} is {}.",
        target, result
    );

    loop_arr();

    loop_label()
}

fn loop_arr() {
    let numbers = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < numbers.len() {
        println!("The value is: {}", numbers[index]);
        index += 1;
    }

    let numbers = [1, 2, 3, 4, 5];

    for number in numbers {
        println!("The value is: {}", number);
    }

    for x in 1..=3 {
        println!("x: {}", x);
    }
}

fn loop_label() {
    let x = 1;

    'outer: loop {
        let mut y = 1;

        'inner: loop {
            if y == 3 {
                y += 1;
                continue 'inner; // Skips to the next iteration of the 'inner loop
            }

            println!("x: {}, y: {}", x, y);

            y += 1;

            if y > 5 {
                break 'outer; // Breaks out of the 'inner loop
            }
        }
    }

    let x = 1;

    let z = 'outer: loop {
        let mut y = 1;

        'inner: loop {
            if y == 3 {
                y += 1;
                continue 'inner; // Skips to the next iteration of the 'inner loop
            }

            println!("x: {}, y: {}", x, y);

            y += 1;

            if y > 5 {
                break 'outer y; // Breaks out of the 'inner loop
            }
        }
    };
    println!("z: {}", z);
}
