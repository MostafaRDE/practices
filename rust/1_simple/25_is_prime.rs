// A program to check if a number is prime or not

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
    let result = way3(&num);
    println!("Result is: {}", result);
}

fn way1(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    let mut s = 0;

    let mut i = 1;
    while i <= *n {
        let r = *n % i;
        if r == 0 {
            s += 1;
        }
        i += 1;
    }

    s == 2
}

fn way2(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    let mut is_prime = true;

    let mut i = 2;
    while i < (*n / 2 + 1) {
        let r = *n % i;
        if r == 0 {
            is_prime = false;
            break;
        }
        i += 1;
    }

    is_prime
}

fn way3(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    let mut is_prime = true;

    for i in 2..(*n / 2 + 1) {
        let r = *n % i;
        if r == 0 {
            is_prime = false;
            break;
        }
    }

    is_prime
}

#[test]
fn test_way1() {
    assert_eq!(way1(&1), false);
    assert_eq!(way1(&2), true);
    assert_eq!(way1(&3), true);
    assert_eq!(way1(&4), false);
}

#[test]
fn test_way2() {
    assert_eq!(way2(&1), false);
    assert_eq!(way2(&2), true);
    assert_eq!(way2(&3), true);
    assert_eq!(way2(&4), false);
}

#[test]
fn test_way3() {
    assert_eq!(way3(&1), false);
    assert_eq!(way3(&2), true);
    assert_eq!(way3(&3), true);
    assert_eq!(way3(&4), false);
}
