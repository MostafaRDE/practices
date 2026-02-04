// A program to print all 2-digit prime numbers

fn main() {
    let result = way1();
    for i in result {
        println!("{}", i);
    }
}

fn is_number_prime(n: &u8) -> bool {
    let mut is_prime = true;

    for i in 2..(*n / 2 + 1) {
        let r = *n % i;
        if r == 0 {
            is_prime = false;
            break;
        }
    }

    is_prime
}

fn way1() -> Vec<u8> {
    let mut result = vec![];
    for i in 10..100 {
        if is_number_prime(&i) {
            result.push(i);
        }
    }

    result
}
