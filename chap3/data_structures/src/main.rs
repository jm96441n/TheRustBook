fn main() {
    // numeric types
    let x = 2.0; // f64
    println!("{}", x);
    
    let y: f32 = 3.0; // f32
    println!("{}", y);

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", remainder);

    // boolean types
    let t = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);

    // word types
    let c = 'z';
    let z = 'ℤ';
    let heart_eye_cat = '😻';

    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eye_cat);

    // Compound types
    //---------------

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:#?}", tup);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // array type
    let a = [1, 2, 3, 4, 5];
    println!("{:#?}", a);

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", 
                  "Sep", "Oct", "Nov", "Dec"];
    println!("{:#?}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:#?}", a);

    let a = [3; 5];
    println!("{:#?}", a);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{}", first);
    println!("{}", second);

    let index = 10;
    let element = a[index]; // invalid index access
    println!("The value of element is: {}", element);
}
