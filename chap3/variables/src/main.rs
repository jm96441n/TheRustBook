// const MAX_POINTS: u32 = 100_000;

fn main() {
    /*
     * Show immutability
     * let x = 5;
     * println!("The value of x is: {}", x);
     * x = 6;
     * println!("The value of x is: {}", x);
     * println!("The value of MAX_POINTS is: {}", MAX_POINTS);
     *
     * Define with mutable variable
     * let mut x = 5;
     * println!("The value of x is: {}", x);
     * x = 6;
     * println!("The value of x is: {}", x);
     *
     * Show how constants can be printed
     * println!("The value of MAX_POINTS is: {}", MAX_POINTS);
     */

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    /*
     * Won't because the type of the variable can't be changed
     * even though the variable is mutable
     * let mut spaces = "    ";
     * spaces = spaces.len();
     */
}
