use regex::Regex;

static QUOTE_PATTERN: &'static str = r#"\{\s+?"quote":.*?\}"#;

#[derive(Deserialize)]
pub struct Quote {
    quote: String,
    length: i32,
    author: String,
    tags: Vec<String>,
    category: String,
    date: String,
    title: String,
    background: String,
    id: String,
}

fn extract_quotes(response_content: &str) -> Vec<String> {
    quote_regex()
        .captures_iter(response_content)
        .filter_map(|capture| capture.at(0).map(|s| s.to_owned()))
        .collect()
}

fn quote_regex() -> Regex {
    Regex::new(QUOTE_PATTERN).unwrap()
}

#[cfg(test)]
mod tests {
    use super::QuoteResponse;
    use serde_json as json;

    #[test]
    fn quote_pattern_works() {
        let response_content = r#"{ "success": { "total": 1 }, "contents": { "quotes": [ { "quote": "Positive anything is better than negative thinking.", "length": "51", "author": "Elbert Hubbard", "tags": [ "inspire", "positive-thinking" ], "category": "inspire", "date": "2016-01-16", "title": "Inspiring Quote of the day", "background": "https://theysaidso.com/img/bgs/man_on_the_mountain.jpg", "id": "g3j5nQxVRTka7Sw3khgdRQeF" }, { "quote": "Positive anything is better than negative thinking.", "length": "51", "author": "Elbert Hubbard", "tags": [ "inspire", "positive-thinking" ], "category": "inspire", "date": "2016-01-16", "title": "Inspiring Quote of the day", "background": "https://theysaidso.com/img/bgs/man_on_the_mountain.jpg", "id": "g3j5nQxVRTka7Sw3khgdRQeF" } ] } }"#;
        let regex = super::quote_regex();
        let captures = regex.captures_iter(&response_content);

        assert!(2 == captures.count());
    }

    #[test]
    fn extract_quotes_returns_expected_response() {
        let response_content = r#"{ "success": { "total": 1 }, "contents": { "quotes": [ { "quote": "Positive anything is better than negative thinking.", "length": "51", "author": "Elbert Hubbard", "tags": [ "inspire", "positive-thinking" ], "category": "inspire", "date": "2016-01-16", "title": "Inspiring Quote of the day", "background": "https://theysaidso.com/img/bgs/man_on_the_mountain.jpg", "id": "g3j5nQxVRTka7Sw3khgdRQeF" }, { "quote": "Positive anything is better than negative thinking.", "length": "51", "author": "Elbert Hubbard", "tags": [ "inspire", "positive-thinking" ], "category": "inspire", "date": "2016-01-16", "title": "Inspiring Quote of the day", "background": "https://theysaidso.com/img/bgs/man_on_the_mountain.jpg", "id": "g3j5nQxVRTka7Sw3khgdRQeF" } ] } }"#;
        let quotes = super::extract_quotes(response_content);
        let parsed_quotes = quotes.iter().filter_map(|quote| json::from_str::<QuoteResponse>(quote).ok());

        assert!(2 == parsed_quotes.count());
    }

    #[test]
    fn read_quote_result_returns_expected_response() {
        let json = r#"{ "quote": "Positive anything is better than negative thinking.", "length": "51", "author": "Elbert Hubbard", "tags": [ "inspire", "positive-thinking" ], "category": "inspire", "date": "2016-01-16", "title": "Inspiring Quote of the day", "background": "https://theysaidso.com/img/bgs/man_on_the_mountain.jpg", "id": "g3j5nQxVRTka7Sw3khgdRQeF" }"#;
        json::from_str::<QuoteResponse>(json).unwrap();
    }
}
