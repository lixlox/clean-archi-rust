use anyhow::Error;
use domain::model::book::Book;
use domain::port::secondary::books::BookRepository;

pub struct InMemoryBookRepository {}

impl BookRepository for InMemoryBookRepository {
    fn get_all(&self) -> Result<Vec<Book>, Error> {
        Ok(self.all_books())
    }
}

impl InMemoryBookRepository {
    pub fn new() -> InMemoryBookRepository {
        InMemoryBookRepository {}
    }

    fn all_books(&self) -> Vec<Book> {
        vec![
            Book {
                id: "1234".to_string(),
                title: "batman#1".to_string(),
                description: "joker war #1".to_string(),
            },
            Book {
                id: "5678".to_string(),
                title: "batman#2".to_string(),
                description: "joker war #2".to_string(),
            },
        ]
    }
}

impl Default for InMemoryBookRepository {
    fn default() -> Self {
        InMemoryBookRepository::new()
    }
}
