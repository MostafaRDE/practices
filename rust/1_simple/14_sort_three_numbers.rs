// A program to sort three numbers in ascending order

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
    println!("Please enter first number:");
    let mut a = read_line_and_convert_it_to_number();

    println!("Please enter second number:");
    let mut b = read_line_and_convert_it_to_number();

    println!("Please enter third number:");
    let mut c = read_line_and_convert_it_to_number();

    way1(&mut a, &mut b, &mut c);
    println!("First number is: {}", a);
    println!("Second number is: {}", b);
    println!("Third number is: {}", c);
}

fn way1(a: &mut i32, b: &mut i32, c: &mut i32) {
    if a > b {
        (*a, *b) = (*b, *a);
    }
    if b > c {
        (*b, *c) = (*c, *b);
    }
    if a > b {
        (*a, *b) = (*b, *a);
    }
}

#[test]
fn test_way1() {
    let mut a = 3;
    let mut b = 2;
    let mut c = 1;
    way1(&mut a, &mut b, & mut c);
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);

    let mut a = -12;
    let mut b = 2;
    let mut c = 1;
    way1(&mut a, &mut b, & mut c);
    assert_eq!(a, -12);
    assert_eq!(b, 1);
    assert_eq!(c, 2);

    let mut a = -12;
    let mut b = 2;
    let mut c = 25;
    way1(&mut a, &mut b, & mut c);
    assert_eq!(a, -12);
    assert_eq!(b, 2);
    assert_eq!(c, 25);

    let mut a = 12;
    let mut b = -2;
    let mut c = 25;
    way1(&mut a, &mut b, & mut c);
    assert_eq!(a, -2);
    assert_eq!(b, 12);
    assert_eq!(c, 25);
}
