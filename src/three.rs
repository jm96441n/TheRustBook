pub fn run() {
    vars();
    shadow();
    tuples();
    arrays();
    with_params(5);
    print_labeled_measurement(5, 'h');
    expression();
    let x = five();
    println!("the value of x is {x}");
    let x = plus_one(5);
    println!("the value of x is {x}");
    if_statement();

    let x = if true { 1 } else { 2 };
    println!("the value of x is {x}");
    let x;
    if true {
        x = 1
    } else {
        x = 2
    };
    println!("the value of x is {x}");
    //    loop_forever();
    count();
    labels();
    while_loop();
    for_loop();
}

// Chapter 3
//// 3.1 Variables/Mutability
fn vars() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
}

fn shadow() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}")
    }
    println!("the value of x is: {x}")
}

//// 3.2 Data Types
fn tuples() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("the value of y is: {y}");

    let five_hundred = tup.0;
    let one = tup.2;
    println!("the value of five_hundred is: {five_hundred}");
    println!("the value of one is: {one}");
}

fn arrays() {
    let t = ([1; 2], [3; 4]);
    let (a, _) = t;
    println!("{:?}", a);
    println!("{:?}", t);
}

//// 3.3 Functions
fn with_params(x: i32) {
    println!("the value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

//// 3.5 Control Flow
fn if_statement() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

//// 3.6 Loops
fn _loop_forever() {
    loop {
        println!("again!");
    }
}

fn count() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is: {result}");
}

fn labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }
        count += 1
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for ele in a {
        println!("the value is {ele}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        println!("{x}");
        sum += x;
    }
    println!("{sum}");
}
