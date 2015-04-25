extern crate currencyReport;

use std::{io, thread};
use std::sync::mpsc::{channel, Sender, Receiver};
use currencyReport::http_get::http_getter;


fn main() {
    let mut x = 0;
    let (tx, rx) = channel();

    thread::spawn(move || http_getter::start_getting_currency(2000, &mut x, &tx));

    loop {
        println!("print from receive {}", rx.recv().unwrap());
    }

    let mut stdin = io::stdin();
    let input = &mut String::new();

    input.clear();
    stdin.read_line(input);

    println!("Hello, world! {}", input);
}
