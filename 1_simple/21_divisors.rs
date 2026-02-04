// A program to calculate and print the divisors of a number n

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

    println!("\nResults:");
    for num in result {
        println!("{}", num);
    }
}

fn way1(n: &u32) -> Vec<u32> {
    let mut divs = vec![];
    let mut i = 1;
    while i <= *n {
        let r = *n % i;
        if r == 0 {
            divs.push(i);
        }
        i += 1;
    }
    divs
}

fn way2(n: &u32) -> Vec<u32> {
    let mut divs = vec![];
    for i in 1..(*n / 2 + 1) {
        if *n % i == 0 {
            divs.push(i);
        }
    }
    divs.push(*n);
    divs
}

#[test]
fn test_way1() {
    assert_eq!(way1(&1), vec![1]);
    assert_eq!(way1(&2), vec![1, 2]);
    assert_eq!(way1(&3), vec![1, 3]);
    assert_eq!(way1(&4), vec![1, 2, 4]);
}

#[test]
fn test_way2() {
    assert_eq!(way2(&1), vec![1]);
    assert_eq!(way2(&2), vec![1, 2]);
    assert_eq!(way2(&3), vec![1, 3]);
    assert_eq!(way2(&4), vec![1, 2, 4]);
}
