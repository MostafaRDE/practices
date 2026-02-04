// A program to calculate and print the sum of digits of a 3-digit number

fn main() {
    let mut input = String::new();
    println!("Please enter a 3-digit number:");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<u32>() {
                Ok(num) => println!("Result is: {}", way1(&num)),
                Err(_) => println!("This is not a number!"),
            }
        },
        Err(_) => println!("Failed to readline"),
    }
}

fn way1(num: &u32) -> u32 {
    let h = *num / 100; // Equals => let h = (*num as f64 / 100.0).floor() as u32;
    let r = *num % 100;
    let t = r / 10; // Equals => let t = (r as f64 / 10.0).floor() as u32;
    let o = r % 10;
    h + t + o
}

#[test]
fn test_way1() {
    assert_eq!(way1(&102), 3);
    assert_eq!(way1(&191), 11);
    assert_eq!(way1(&654), 15);
    assert_eq!(way1(&999), 27);
}
