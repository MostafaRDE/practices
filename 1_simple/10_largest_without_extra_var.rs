// A program to find and print the largest of three numbers without using an extra variable

fn read_line_and_convert_it_to_number() -> u32 {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<u32>() {
                Ok(num) => num,
                Err(_) => panic!("This is not a number"),
            }
        },
        Err(_) => panic!("Failed to readline"),
    }
}

fn main() {
    println!("Please enter first number:");
    let a = read_line_and_convert_it_to_number();

    println!("Please enter second number:");
    let b = read_line_and_convert_it_to_number();

    println!("Please enter third number:");
    let c = read_line_and_convert_it_to_number();

    let m = way1(&a, &b, &c);
    println!("Maximum inputted numbers is: {}", m);
}

fn way1(a: &u32, b: &u32, c: &u32) -> u32 {
    if a > b {
        if a > c {
            *a
        } else {
            *c
        }
    } else if b > c {
        *b
    } else {
        *c
    }
}

#[test]
fn test_way1() {
    assert_eq!(way1(&1, &2, &3), 3);
    assert_eq!(way1(&3, &2, &1), 3);
    assert_eq!(way1(&2, &3, &1), 3);
}
