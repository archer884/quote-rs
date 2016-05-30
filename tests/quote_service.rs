extern crate dotenv;
extern crate quote_rs;

use std::env;
use dotenv::dotenv;
use quote_rs::{Query, Service};

#[test]
#[ignore]
fn qod() {
    let response = get_service().qod();
    
    assert!(response.is_ok(), format!("{:#?}", response));
}

#[test]
#[ignore]
fn qod_for_category() {
    let response = get_service().qod_for_category("sports");
    
    assert!(response.is_ok(), format!("{:#?}", response));
}

#[test]
#[ignore]
fn random() {
    let response = get_service().random();
    
    assert!(response.is_ok(), format!("{:#?}", response));
}

#[test]
#[ignore]
fn by_id() {
    let response = get_service().by_id("g3j5nQxVRTka7Sw3khgdRQeF");
    
    assert!(response.is_ok(), format!("{:#?}", response));
}

#[test]
#[ignore]
fn query() {
    let response = get_service().query(&Query::new().by_author("Orson Welles"));
    
    assert!(response.is_ok(), format!("{:#?}", response));
}

#[test]
#[ignore]
fn categories() {
    let response = get_service().categories(100);
    
    assert!(response.is_ok(), format!("{:#?}", response));
}

#[test]
#[ignore]
fn image() {
    let response = get_service().image();
    
    assert!(response.is_ok(), format!("{:#?}", response));
}

#[test]
#[ignore]
fn image_query() {
    let response = get_service().image_query(&Query::new().by_category("inspire"));
    
    assert!(response.is_ok(), format!("{:#?}", response));
}

fn get_service() -> Service {
    dotenv().ok();
    
    let api_key = env::var("QUOTE_API_KEY").expect("API key must be set");
    println!("API_KEY: {}", api_key);
    
    Service::with_key(api_key)
}