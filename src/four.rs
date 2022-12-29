pub fn run() {
    scope_fun();
    strings();
    data_interaction();
    its_cloning_time();
    ownership_and_functions();
    ref_len();
    mut_ref();
}

//// 4.1 What is ownership?
fn scope_fun() {
    let s = "hello";
    {
        let s = "inside scope";
        println!("{s}")
    }
    println!("{s}")
}

fn strings() {
    let mut s = String::from("hello");
    println!("{s}");
    s.push_str(", world!");
    println!("{s}");
}

fn data_interaction() {
    let x = 5;
    let y = x;
    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");
}

fn its_cloning_time() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership_and_functions() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s1);
    println!("{}", s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

//// 4.2 References and Borrowing

fn ref_len() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mut_ref() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
