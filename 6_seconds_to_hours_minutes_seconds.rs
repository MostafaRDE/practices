// A program to convert seconds into hours, minutes, and seconds format

fn main() {
    let mut input = String::new();
    println!("Please enter a number of seconds:");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<u32>() {
                Ok(num) => println!("Result is: {}", way1(&num)),
                Err(_) => println!("This is not a number"),
            }
        },
        Err(_) => println!("Failed to readline"),
    }
}

fn way1(num : &u32) -> String {
    let h = *num / 3600;
    let r = *num % 3600;
    let m = r / 60;
    let s = r % 60;
    format!("{h}:{m:0>2}:{s:0>2}")
} 

#[test]
fn test_way1() {
    assert_eq!(way1(&552), "0:09:12");
    assert_eq!(way1(&3600), "1:00:00");
    assert_eq!(way1(&259200), "72:00:00")
}
