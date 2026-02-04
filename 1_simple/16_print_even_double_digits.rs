// A program to print even double-digit numbers

fn main() {
    for i in 10..100 {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}
