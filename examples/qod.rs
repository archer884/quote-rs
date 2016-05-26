extern crate quoters;

use quoters::Service;

fn main() {
    let mut service = Service::new();
    match service.qod() {
        Err(e) => println!("{}", e),
        Ok(quote) => println!("{}", quote.quote),
    }
}