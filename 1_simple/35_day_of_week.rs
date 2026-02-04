// A program to determine the day of the week for the nth day of a year starting from Wednesday

fn read_line_and_convert_it_to_number() -> u16 {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Err(_) => panic!("Failed to readline"),
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<u16>() {
                Err(_) => panic!("This is not a number"),
                Ok(num) => num,
            }
        },
    }
}

fn main() {
    println!("Please enter the day number:");
    let day_number = read_line_and_convert_it_to_number();
    let result = way1(&day_number);
    println!("Salary without tax: {:?}", result);
}

#[derive(Debug, PartialEq)]
enum Days {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 0,
}

fn way1(n: &u16) -> Days {
    let d = *n % 7;
    match d {
        1 => Days::Wednesday,
        2 => Days::Thursday,
        3 => Days::Friday,
        4 => Days::Saturday,
        5 => Days::Sunday,
        6 => Days::Monday,
        _ => Days:: Tuesday,
    }
}

#[test]
fn test_way1() {
    assert_eq!(way1(&1), Days::Wednesday);
    assert_eq!(way1(&9), Days::Thursday);
    assert_eq!(way1(&14), Days::Tuesday);
}
