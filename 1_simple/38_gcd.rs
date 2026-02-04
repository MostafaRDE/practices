// A program to calculate and print the greatest common divisor (GCD) of a number n

fn read_line_and_convert_it_to_number() -> u32 {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Err(_) => panic!("Failed to readline"),
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<u32>() {
                Err(_) => panic!("This is not a number"),
                Ok(num) => num,
            }
        },
    }
}

fn main() {
    println!("Please enter \"a\" number:");
    let mut a = read_line_and_convert_it_to_number();

    println!("Please enter \"b\" number:");
    let mut b = read_line_and_convert_it_to_number();

    let result = way1(&mut a, &mut b);
    println!("Result is: {}", result);
}

fn way1(m: &mut u32, n: &mut u32) -> u32 {
    if *m > *n {
        (*m, *n) = (*n, *m);
    }

    let mut r = *m % *n;
    while r != 0 {
        *m = *n;
        *n = r;
        r = *m % *n;
    }
    *n
}
