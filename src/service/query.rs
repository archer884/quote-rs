#[derive(Debug)]
pub struct Query {
    min: Option<i32>,
    max: Option<i32>,
    category: Option<String>,
    author: Option<String>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            min: None,
            max: None,
            category: None,
            author: None,
        }
    }

    pub fn with_min(mut self, min: i32) -> Query {
        self.min = Some(min);
        self
    }

    pub fn with_max(mut self, max: i32) -> Query {
        self.max = Some(max);
        self
    }

    pub fn by_category<T: Into<String>>(mut self, category: T) -> Query {
        self.category = Some(category.into());
        self
    }

    pub fn by_author<T: Into<String>>(mut self, author: T) -> Query {
        self.author = Some(author.into());
        self
    }
}

impl Default for Query {
    fn default() -> Self {
        Query::new()
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        fn encode(input: &str) -> String {
            input.chars().fold(String::new(), |mut buf, c| {
                match c {
                    ' ' => buf.push_str("%20"),
                    '%' => buf.push_str("%25"),
                    '"' => buf.push_str("%22"),
                    '#' => buf.push_str("%23"),
                    '<' => buf.push_str("%3c"),
                    '>' => buf.push_str("%3e"),
                    c => buf.push(c),  
                };
                
                buf
            })
        }
        
        let mut parts = Vec::with_capacity(4);
        
        if let Some(min) = self.min {
            parts.push(format!("minlength={}", min));
        }
        
        if let Some(max) = self.max {
            parts.push(format!("maxlength={}", max));
        }
        
        if let Some(ref category) = self.category {
            parts.push(format!("category={}", encode(category)));
        }
        
        if let Some(ref author) = self.author {
            parts.push(format!("author={}", encode(author)));
        }
        
        parts.join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::Query;

    #[test]
    fn query_produces_expected_results_for_min_and_max() {
        let query = Query::new().with_min(20);
        assert_eq!("minlength=20", &query.to_string());

        let query = Query::new().with_max(20);
        assert_eq!("maxlength=20", &query.to_string());

        let query = Query::new().with_min(20).with_max(20);
        assert_eq!("minlength=20&maxlength=20", &query.to_string());
    }

    #[test]
    fn query_produces_expected_results_for_category() {
        let query = Query::new().by_category("testing");
        assert_eq!("category=testing", &query.to_string());

        let query = Query::new().by_category("testing").with_min(20).with_max(20);
        assert_eq!("minlength=20&maxlength=20&category=testing", &query.to_string());
    }

    #[test]
    fn query_produces_expected_results_for_author() {
        let query = Query::new().by_author("testing");
        assert_eq!("author=testing", &query.to_string());

        let query = Query::new().with_min(100).with_max(100).by_author("testing");
        assert_eq!("minlength=100&maxlength=100&author=testing", &query.to_string());
    }
}