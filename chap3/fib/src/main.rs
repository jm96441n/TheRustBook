fn main() {
    let num = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("N    Fib");
    for n in num.iter() {
        println!("{}   {}", n, fib(*n));
    };
}

fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}
