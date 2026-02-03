// A program to print odd numbers between two given numbers a and b

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
        if i % 2 == -1 || i % 2 == 1 {
            result.push(i);
        }
    }

    result
}

#[test]
fn test_way1() {
    assert_eq!(
        way1(&mut 12, &mut -12),
        vec![-11, -9, -7, -5, -3, -1, 1, 3, 5, 7, 9, 11]
    );
    assert_eq!(
        way1(&mut -6, &mut 4),
        vec![-5, -3, -1, 1, 3]
    );
    assert_eq!(
        way1(&mut -5, &mut 3),
        vec![-3, -1, 1]
    );
    assert_eq!(
        way1(&mut 5, &mut -3),
        vec![-1, 1, 3]
    );
}
