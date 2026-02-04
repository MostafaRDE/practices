// A program to convert a number into day, month, and year format

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

    let (y, m, d) = way1(&num);
    println!("Result is: {}-{:0>2}-{:0>2}", y, m, d);
}

fn way1(n: &u32) -> (u32, u32, u32) {
    let y = *n / 360;
    let r = *n % 360;
    let m = r / 30;
    let d = r % 30;
    (y, m, d)
}

#[test]
fn test_way1() {
    assert_eq!(way1(&0), (0, 0, 0));
    assert_eq!(way1(&10), (0, 0, 10));
    assert_eq!(way1(&31), (0, 1, 1));
    assert_eq!(way1(&65), (0, 2, 5));
    assert_eq!(way1(&359), (0, 11, 29));
    assert_eq!(way1(&360), (1, 0, 0));
    assert_eq!(way1(&362), (1, 0, 2));
}
