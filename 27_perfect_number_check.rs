// A program to check if a number is perfect or not

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
    let result = way2(&num);
    println!("Result is: {}", result);
}

fn way1(n: &u32) -> bool {
    let mut s = 0;

    let mut i = 1;
    while i <= *n / 2 {
        let r = *n % i;
        if r == 0 {
            s += i;
        }
        i += 1;
    }

    s == *n
}

fn way2(n: &u32) -> bool {
    let mut s = 0;

    for i in 1..(*n / 2 + 1) {
        let r = *n % i;
        if r == 0 {
            s += i;
        }
    }

    s == *n
}

#[test]
fn test_way1() {
    assert_eq!(way1(&2), false);
    assert_eq!(way1(&3), false);
    assert_eq!(way1(&4), false);
    assert_eq!(way1(&5), false);
    assert_eq!(way1(&6), true);
}

#[test]
fn test_way2() {
    assert_eq!(way2(&2), false);
    assert_eq!(way2(&3), false);
    assert_eq!(way2(&4), false);
    assert_eq!(way2(&5), false);
    assert_eq!(way2(&6), true);
}
