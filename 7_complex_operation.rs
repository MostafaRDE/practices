// A program to perform a complex operation with a 6-digit number (multiply first two digits, subtract second two from third, divide by fourth, sum the results)

fn main() {
    let mut input = String::new();
    println!("Please enter a 6-digit number:");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<i32>() {
                Ok(num) => println!("Result is: {}", way1(&num)),
                Err(_) => println!("This is not a number"),
            }
        },
        Err(_) => println!("Failed to readline"),
    }
}

fn way1(num: &i32) -> i32 {
    let d1 = *num / 100_000;
    let r1 = *num % 100_000;
    let d2 = r1 / 10_000;
    let r2 = r1 % 10_000;
    let d3 = r2 / 1000;
    let r3 = r2 % 1000;
    let d4 = r3 / 100;
    let r4 = r3 % 100;
    let d5 = r4 / 10;
    let d6 = r4 % 10;

    d1 * d2 + (d3 - d4) + d5 / d6
}

#[test]
fn test_way1() {
    assert_eq!(way1(&123444), 2);
    assert_eq!(way1(&987654), 74);
    assert_eq!(way1(&987684), 75);
}
