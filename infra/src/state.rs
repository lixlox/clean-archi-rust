use crate::db::in_memory::InMemoryBookRepository;
use crate::web::routes::WebState;
use domain::usecase::book_query::BookQueryUseCase;
use std::sync::Arc;

pub async fn state_factory() -> std::io::Result<WebState> {
    let book_repository = InMemoryBookRepository::new();
    let query_books = BookQueryUseCase::new(Box::new(book_repository));
    let state = WebState {
        query_books: Arc::new(query_books),
    };
    Ok(state)
}
