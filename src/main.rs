mod five;
mod four;
mod three;
mod two;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let chap = &args[1];
    if chap == "2" {
        two::run();
    } else if chap == "3" {
        three::run();
    } else if chap == "4" {
        four::run();
    } else if chap == "5" {
        five::run();
    }
}
