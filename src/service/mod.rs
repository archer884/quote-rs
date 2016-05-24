use model::category::Categories;
use model::quote::{Quote, Quotes};

pub type Result<T> = ::std::result::Result<T, ServiceError>;

pub enum ServiceError {}

// API Secret: X-TheySaidSo-Api-Secret
pub trait QuoteService {
    // GET: http://quotes.rest/qod.json
    fn qod(&mut self) -> Result<Quote>;

    // GET: http://quotes.rest/qod/categories.json
    fn qod_categories(&mut self) -> Result<Categories>;

    // GET: http://quotes.rest/qod/qod.json?category=testing
    fn qod_for_category(&mut self, category: &str) -> Result<Quote>;

    // GET: http://quotes.rest/quote.json
    fn random(&mut self) -> Result<Quote>;

    // GET: http://quotes.rest/quote.json?minlength=100&maxlength=300
    fn query(&mut self, Query) -> Result<Quotes>;

    // GET: http://quotes.rest/quote/categories.json
    fn categories(&mut self) -> Result<Categories>;
}