// A program to print the Fibonacci series up to a number n

fn main() {
    let mut f1 = 1;
    let mut f2 = 1;
    let mut f3 = f1 + f2;

    println!("{}", f1);
    println!("{}", f2);

    while f3 < 1000 {
        println!("{}", f3);
        
        f1 = f2;
        f2 = f3;
        f3 = f1 + f2;
    }
}
