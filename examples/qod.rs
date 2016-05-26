extern crate quote_rs;

use quote_rs::Service;

fn main() {
    let mut service = Service::new();
    match service.qod() {
        Err(e) => println!("{}", e),
        Ok(quote) => println!("{}", quote.quote),
    }
}