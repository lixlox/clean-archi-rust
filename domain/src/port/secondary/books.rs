use crate::model::book::Book;
use anyhow::Error;
use mockall::*;

#[automock]
pub trait BookRepository {
    fn get_all(&self) -> Result<Vec<Book>, Error>;
}
