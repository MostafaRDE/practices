// A program to check if the reversed 3-digit number is equal to the original number

fn main() {
    let mut input = String::new();
    println!("Please enter a 3-digit number:");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<u32>() {
                Ok(num) => println!("Result is: {}", way1(&num)),
                Err(_) => println!("This is not a number"),
            }
        },
        Err(_) => println!("Failed to readline"),
    }
}

fn way1(num: &u32) -> bool {
    let h = *num / 100; // Equals. => let h = (*num as f64 / 100.0).floor() as u32;
    let r = *num % 100;
    let t = r / 10; // Equals => let t = (r as f64 / 10.0).floor() as u32;
    let o = r % 10;
    let revered_num = o * 100 + t * 10 + h;
    *num == revered_num
}

#[test]
fn test_way1() {
    assert_eq!(way1(&100), false);
    assert_eq!(way1(&202), true);
    assert_eq!(way1(&545), true);
    assert_eq!(way1(&987), false);
}
