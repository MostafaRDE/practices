// A program to print multiples of 3 among three given numbers

fn main() {
    for i in 100..1000 {
        if i % 3 == 0 {
            println!("{}", i);
        }
    }
}
