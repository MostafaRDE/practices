// A program to calculate and print the factorial of a number n

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
    println!("Please enter a number:");
    let num = read_line_and_convert_it_to_number();
    let result = way1(&num);
    println!("Result is: {}", result);
}

fn way1(n: &u32) -> u32 {
    let mut i = 1;
    let mut s = 1;
    while i <= *n {
        s *= i;
        i += 1;
    }
    s
}

#[test]
fn test_way1() {
    assert_eq!(way1(&1), 1);
    assert_eq!(way1(&2), 2);
    assert_eq!(way1(&3), 6);
    assert_eq!(way1(&4), 24);
    assert_eq!(way1(&5), 120);
}
