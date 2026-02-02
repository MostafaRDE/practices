// A program to print multiples of 3 among three given numbers

fn main() {
    way2();
}

fn _way1() {
    for i in 100..1000 {
        if i % 3 == 0 {
            println!("{}", i);
        }
    }
}

fn way2() {
    for i in (102..1000).step_by(3) {
        println!("{}", i);
    }
}
