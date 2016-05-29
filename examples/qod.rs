extern crate quote_rs;

use quote_rs::Service;

fn main() {
    let service = match std::env::args().nth(1) {
        None => Service::new(),
        Some(key) => Service::with_key(key),
    };
    
    match service.qod() {
        Err(e) => println!("{}", e),
        Ok(quote) => println!("{}", quote.quote),
    }
}