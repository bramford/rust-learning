fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    let _x = [(1 as i32),2,3,4];
    let _y = [1,2,3,4 as i32];
    let _z = [_x,_x].concat();

    // Functional approach with range
    let sum_of_squared_odd_numbers : u32 =
        (1..5 as u32).map(|n| n * n)                             // All natural numbers squared
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
             .filter(|&n_squared| is_odd(n_squared))     // That are odd
             .sum();                                     // Sum them
    println!("functional style from range: {}", sum_of_squared_odd_numbers);
                                                         //
    // Functional approach with array
    let sum_of_squared_odd_numbers_first : u32 =
        [1,2,3,4 as u32].iter().map(|n| n * n)                             // All natural numbers squared
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
             .filter(|&n_squared| is_odd(n_squared))     // That are odd
             .sum();                                     // Sum them
                                                         //
    println!("functional style from array: {}", sum_of_squared_odd_numbers_first);
}

