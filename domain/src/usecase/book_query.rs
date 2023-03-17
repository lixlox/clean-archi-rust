use crate::model::book::Book;
use crate::port::{primary::query::BookQueryPort, secondary::books::BookRepository};
use anyhow::Error;

pub struct BookQueryUseCase {
    pub book_repository: Box<dyn BookRepository>,
}

impl BookQueryPort for BookQueryUseCase {
    fn get_all(&self) -> Result<Vec<Book>, Error> {
        self.book_repository.get_all()
    }
}

impl BookQueryUseCase {
    pub fn new(book_repository: Box<dyn BookRepository>) -> BookQueryUseCase {
        BookQueryUseCase { book_repository }
    }
}
