// A program to print "Mostafa" n times

fn read_line_and_convert_it_to_number() -> i32 {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Err(_) => panic!("Failed to readline"),
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<i32>() {
                Err(_) => panic!("This is not a number"),
                Ok(num) => num,
            }
        },
    }
}

fn main() {
    println!("Please enter a number:");
    let n = read_line_and_convert_it_to_number();

    if n < 1 {
        panic!("Number must be greater than zero");
    }

    let mut counter = 0;
    while counter < n {
        println!("Mostafa");
        counter += 1;
    }
}
