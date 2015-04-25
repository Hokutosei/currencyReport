use utils::utils;
use std::sync::mpsc::{Sender};
use std::{thread};

pub fn test(c: &Sender<&str>) {
    println!("called!")
}

fn http_get() {
    
}

pub fn start_getting_currency(loop_delay: u32, x: &mut u32, c: &Sender<&str>) {
    loop {
        utils::counter(x);
        println!("{}", x);
        thread::sleep_ms(loop_delay);

        c.send("test");
    }
}
