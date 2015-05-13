extern crate hyper;

use std::io::Read;

use self::hyper::Client;

use self::hyper::header::Connection;

use utils::utils;
use std::sync::mpsc::{Sender};
use std::{thread};

pub fn test(c: &Sender<&str>) {
    println!("called!")
}

pub fn start_getting_currency(loop_delay: u32, x: &mut u32, c: &Sender<String>, msg: &str) {
    loop {
        utils::counter(x);
        thread::sleep_ms(loop_delay);

        let msg_str = format!("a thread {}, counter: {}", msg, x);
        println!("{}", msg_str);

        http_get();

        c.send(msg_str);
    }
}



pub fn construct_url(c: &Sender<String>, city_code: &str) {
        let url_str = format!("http://api.openweathermap.org/data/2.5/weather?q={}", city_code);
        println!("{}", url_str);
        c.send(url_str);
}


fn http_get() {
    let mut client = Client::new();

    let url = "http://www.freecurrencyconverterapi.com/api/v3/convert?q=JPY_PHP&compact=y";

    let mut res = client.get(url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("response: {}", body);
}
