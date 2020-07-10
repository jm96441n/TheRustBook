fn main() {
    let temps: [f64; 11] = [0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];

    println!("CELSIUS    FAHRENHEIT");
    for ele in temps.iter() {
        println!("{}     {}", ele, celsius_to_fahrenheit(*ele));
    }

    println!("FAHRENHEIT    CELSIUS");
    for ele in temps.iter() {
        println!("{}     {}", ele, fahrenheit_to_celsius(*ele));
    }

}

fn celsius_to_fahrenheit(cel: f64) -> f64 {
    (cel * (9.0/5.0)) + 32.0
}

fn fahrenheit_to_celsius(fahr: f64) -> f64 {
    (fahr - 32.0) * (5.0/9.0)
}
