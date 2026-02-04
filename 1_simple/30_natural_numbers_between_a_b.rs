// A program to print natural numbers between two given numbers a and b

fn read_line_and_convert_it_to_number() -> f64 {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Err(_) => panic!("Failed to readline"),
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<f64>() {
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

    println!("\n");
    let (x, y) = way1(&a, &b);
    for i in x..y {
        println!("{}", i)
    }
}

fn way1(a: &f64, b: &f64) -> (u32, u32) {
    if !(*a >= 1.0 || *b >= 1.0 || a == b) {
        return (0, 0);
    }

    let mut x = a.floor() as u32;
    let mut y = b.floor() as u32;
    if *a == *b {
        return (0, 0);
    }
    else if x > y {
        x = x + y;
        y = x - y;
        x = x - y;
    }

    (if x < 1 { 1 } else { x + 1 }, y)
}

#[test]
fn test_way1() {
    assert_eq!(way1(&25.44, &-1.25), (1, 25));
    assert_eq!(way1(&-1.25, &25.44), (1, 25));
    assert_eq!(way1(&5.0, &55.0), (6, 55));
    assert_eq!(way1(&0.0, &90.0), (1, 90));
    assert_eq!(way1(&0.5, &90.0), (1, 90));
    assert_eq!(way1(&1.0, &90.0), (2, 90));
    assert_eq!(way1(&4.0, &90.0), (5, 90));
}
