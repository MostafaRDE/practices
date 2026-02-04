// A program to sum the series of numbers from 1 to n

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
    let num = read_line_and_convert_it_to_number();
    let result = way1(&num);
    println!("Result is: {}", result);
}

fn way1(n: &i32) -> u32 {
    let mut i: i32 = 1;
    let mut s: u32 = 0;
    while i <= *n {
        s += i as u32;
        i += 1;
    }
    s
}

#[test]
fn test_way1() {
    assert_eq!(way1(&-2), 0);
    assert_eq!(way1(&0), 0);
    assert_eq!(way1(&1), 1);
    assert_eq!(way1(&2), 3);
    assert_eq!(way1(&5), 15);
}
