// A program to calculate and print the sum of digits of a 2-digit number

fn main() {
    let mut input = String::new();
    println!("Please enter a 2-digit number:");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<u32>() {
                Ok(num) => println!("Result is: {}", way1(&num)),
                Err(_) => println!("That is not a number!"),
            }
        }
        Err(_) => println!("Failed to readline"),
    }
}

fn way1(num: &u32) -> u32 {
    let t = *num / 10; // Equals => let t = (*num as f64 / 10.0).floor() as u32;
    let o = *num % 10;
    t + o
}

#[test]
fn test_way1() {
    assert_eq!(way1(&12), 3);
    assert_eq!(way1(&19), 10);
    assert_eq!(way1(&65), 11);
    assert_eq!(way1(&99), 18);
}
