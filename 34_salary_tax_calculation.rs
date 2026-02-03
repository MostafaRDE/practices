// A program to calculate and print the net salary and tax of an employee given their gross salary

/**
 * [0%] salary <= 800
 * [5%] 800 < salary <= 1200
 * [7%] 1200 < salary <= 2000
 * [10%] 2000 < salary
 */


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
    println!("Please enter salary:");
    let salary = read_line_and_convert_it_to_number();
    let result = way1(&salary);
    println!("Salary without tax: {}", result);
}

fn way1(salary: &u32) -> f64 {
    let mut temp_salary = *salary;
    let mut tax = 0.0;

    if temp_salary > 2000 {
        let temp = temp_salary - 2000;
        tax += temp as f64 * 0.1;
        temp_salary -= temp;
    }
    if temp_salary > 1200 {
        let temp = temp_salary - 1200;
        tax += temp as f64 * 0.07;
        temp_salary -= temp;
    }
    if temp_salary > 800 {
        let temp = temp_salary - 800;
        tax += temp as f64 * 0.05;
    }

    *salary as f64 - tax
}

#[test]
fn test_way1() {
    assert_eq!(way1(&600), 600.0);
    assert_eq!(way1(&800), 800.0);
    assert_eq!(way1(&900), 895.0);
    assert_eq!(way1(&1200), 1180.0);
    assert_eq!(way1(&1300), 1273.0);
    assert_eq!(way1(&2000), 1924.0);
    assert_eq!(way1(&2100), 2014.0);
}
