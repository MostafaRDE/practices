// A program to calculate and print the sum of digits of all double-digit numbers

fn main() {
    way2();
}

fn _way1() {
    let mut s = 0;
    let mut i = 10;
    while i < 100 {
        s += i;
        i += 1;
    }
    println!("{}", s);
}

fn way2() {
    let mut s = 0;
    for i in 10..100 {
        s += i;
    }
    println!("{}", s);
}
