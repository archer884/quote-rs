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

    pub fn with_category<T: Into<String>>(mut self, category: T) -> Query {
        self.category = Some(category.into());
        self
    }

    pub fn with_author<T: Into<String>>(mut self, author: T) -> Query {
        self.author = Some(author.into());
        self
    }

    pub fn append_to_buffer(&self, buf: &mut String) {
        use std::fmt::Write;
        
        let mut has_written = false;
        
        if let Some(min) = self.min {
            write!(buf, "minlength={}", min).ok();
            has_written = true;
        }
        
        if let Some(max) = self.max {
            if has_written {
                buf.push('&');
            }
            
            write!(buf, "maxlength={}", max).ok();
            has_written = true;
        }
        
        if let Some(ref category) = self.category {
            if has_written {
                buf.push('&');
            }
            
            write!(buf, "category={}", category).ok();
            has_written = true;
        }
        
        if let Some(ref author) = self.author {
            if has_written {
                buf.push('&');
            }
            
            write!(buf, "author={}", author).ok();
        }
    }
}

impl Default for Query {
    fn default() -> Self {
        Query::new()
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        self.append_to_buffer(&mut buf);
        buf
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
        let query = Query::new().with_category("testing");
        assert_eq!("category=testing", &query.to_string());

        let query = Query::new().with_category("testing").with_min(20).with_max(20);
        assert_eq!("minlength=20&maxlength=20&category=testing", &query.to_string());
    }

    #[test]
    fn query_produces_expected_results_for_author() {
        let query = Query::new().with_author("testing");
        assert_eq!("author=testing", &query.to_string());

        let query = Query::new().with_min(100).with_max(100).with_author("testing");
        assert_eq!("minlength=100&maxlength=100&author=testing", &query.to_string());
    }
}