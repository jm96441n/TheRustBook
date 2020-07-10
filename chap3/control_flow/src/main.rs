fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let new_num = 3;

    if new_num != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let cond = true;
    let number = if cond { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // Repeating with `loop`
    let mut i: i32 = 0;
    loop {
        println!("again!");
        if i > 2 {
            break;
        }
        i += 1;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Conditional Loops with `while`

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through collection with `for`
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    // can be unsafe due to iterating out of array indexes
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // use for loop with a Range to do some a set number of times
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
