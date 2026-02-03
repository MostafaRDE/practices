// A program to reverse the digits of a number n

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
    let result = way1(&num);
    println!("Reverse is: {}", result);
}

fn way1(n: &u32) -> u32 {
    let mut t = *n;
    let mut s = 0;
    while t != 0 {
        let o = t % 10;
        s = o + s * 10;
        t /= 10;
    }
    s
}

#[test]
fn test_way1() {
    assert_eq!(way1(&0), 0);
    assert_eq!(way1(&1), 1);
    assert_eq!(way1(&5), 5);
    assert_eq!(way1(&10), 1);
    assert_eq!(way1(&100), 1);
    assert_eq!(way1(&110), 11);
    assert_eq!(way1(&101), 101);
    assert_eq!(way1(&194827), 728491);
}
