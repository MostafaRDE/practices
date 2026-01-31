// A program to check if a number is even or odd

fn read_line_and_convert_it_to_number() -> i32 {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<i32>() {
                Ok(num) => num,
                Err(_) => panic!("This is not a number"),
            }
        },
        Err(_) => panic!("Failed to readline"),
    }
}

fn main() {
    println!("Please enter a number:");
    let num = read_line_and_convert_it_to_number();

    let result = way1(&num);
    println!("Number is even? {}", result);
}

fn way1(num: &i32) -> bool {
    num % 2 == 0
}

#[test]
fn test_way1() {
    assert_eq!(way1(&4), true);
    assert_eq!(way1(&5), false);
}
