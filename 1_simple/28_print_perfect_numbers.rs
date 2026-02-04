// A program to print all perfect numbers under 1000

fn main() {
    for i in 1..1000 {
        if is_number_perfect(&i) {
            println!("{}", i);
        }
    }
}

fn is_number_perfect(n: &u32) -> bool {
    let mut s = 0;

    for i in 1..(*n / 2 + 1) {
        let r = *n % i;
        if r == 0 {
            s += i;
        }
    }

    s == *n
}
