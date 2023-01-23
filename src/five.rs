pub fn run() {
    let mut s = String::from("hello world");
    let i = first_words(&s);
    let i2 = second_words(&s);
    println!("{}", i);
    println!("{}", i2);
    s.clear();
}

fn first_words(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_words(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_idx = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_idx == 0 {
                first_idx = i + 1;
            } else {
                return &s[first_idx..i];
            }
        }
    }
    return &s[first_idx..];
}
