use anyhow::Error;

use crate::model::book::Book;

pub trait BookQueryPort {
    fn get_all(&self) -> Result<Vec<Book>, Error>;
}
