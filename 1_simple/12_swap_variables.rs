// A program to swap the values of two variables

fn read_line_and_convert_it_to_number() -> i32 {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<i32>() {
                Ok(num) => num,
                Err(_) => panic!("This is not a number"),
            }
        }
        Err(_) => panic!("Failed to readline"),
    }
}

fn main() {
    println!("Please enter first number:");
    let mut a = read_line_and_convert_it_to_number();

    println!("Please enter second number:");
    let mut b = read_line_and_convert_it_to_number();

    way1(&mut a, &mut b);
    println!("After swapping first number is: {}", a);
    println!("After swapping second number is: {}", b);
}

fn way1(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

#[test]
fn test_way1() {
    let mut a = 12;
    let mut b = 24;
    way1(&mut a, &mut b);
    assert_eq!(a, 24);
    assert_eq!(b, 12);

    let mut a = -12;
    let mut b = -24;
    way1(&mut a, &mut b);
    assert_eq!(a, -24);
    assert_eq!(b, -12);

    let mut a = 12;
    let mut b = -24;
    way1(&mut a, &mut b);
    assert_eq!(a, -24);
    assert_eq!(b, 12);
}
