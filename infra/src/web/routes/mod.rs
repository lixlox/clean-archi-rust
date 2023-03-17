use domain::port::primary::query::BookQueryPort;
use std::sync::Arc;

pub mod books;
pub mod health_check;

#[derive(Clone)]
pub struct WebState {
    pub query_books: Arc<dyn BookQueryPort>,
}
