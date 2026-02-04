// A program to check if three sides can form a right-angled triangle

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
    let a = read_line_and_convert_it_to_number();

    println!("Please enter \"b\" number:");
    let b = read_line_and_convert_it_to_number();

    println!("Please enter \"c\" number:");
    let c = read_line_and_convert_it_to_number();

    let result = way1(&a, &b, &c);
    println!("Result is: {}", result);
}

fn way1(a: &u32, b: &u32, c: &u32) -> bool {
    a.pow(2) == b.pow(2) + c.pow(2)
        || b.pow(2) == a.pow(2) + c.pow(2)
        || c.pow(2) == a.pow(2) + b.pow(2)
}

#[test]
fn test_way1() {
    assert_eq!(way1(&4, &3, &5), true);
    assert_eq!(way1(&1, &2 ,&3), false);
}
