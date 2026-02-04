// A program to calculate and print the sum of the divisors of a number n

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
    let mut sum = 0;

    let mut i = 1;
    while i <= *n {
        let r = *n % i;
        if r == 0 {
            sum += i;
        }
        i += 1;
    }

    sum
}

fn way2(n: &u32) -> u32 {
    let mut sum = 0;

    for i in 1..(*n / 2 + 1) {
        let r = *n % i;
        if r == 0 {
            sum += i;
        }
    }
    sum += *n;

    sum
}

#[test]
fn test_way1() {
    assert_eq!(way1(&1), 1);
    assert_eq!(way1(&2), 3);
    assert_eq!(way1(&3), 4);
    assert_eq!(way1(&4), 7);
}

#[test]
fn test_way2() {
    assert_eq!(way2(&1), 1);
    assert_eq!(way2(&2), 3);
    assert_eq!(way2(&3), 4);
    assert_eq!(way2(&4), 7);
}
