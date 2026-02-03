// A program to print even numbers between two given numbers a and b

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
    println!("Please enter \"a\" number:");
    let mut a = read_line_and_convert_it_to_number();

    println!("Please enter \"b\" number:");
    let mut b = read_line_and_convert_it_to_number();

    println!("\n");
    let result = way1(&mut a, &mut b);
    for i in result {
        println!("{}", i);
    }
}

fn way1(a: &mut i32, b: &mut i32) -> Vec<i32> {
    let mut result = vec![];

    if *a > *b {
        (*a, *b) = (*b, *a)
    }

    for i in (*a + 1)..*b {
        if i % 2 == 0 {
            result.push(i);
        }
    }

    result
}

#[test]
fn test_way1() {
    assert_eq!(
        way1(&mut 12, &mut -12),
        vec![-10, -8, -6, -4, -2, 0, 2, 4, 6, 8, 10]
    );
    assert_eq!(
        way1(&mut -6, &mut 4),
        vec![-4, -2, 0, 2]
    );
    assert_eq!(
        way1(&mut -5, &mut 3),
        vec![-4, -2, 0, 2]
    );
    assert_eq!(
        way1(&mut 5, &mut -3),
        vec![-2, 0, 2, 4]
    );
}
