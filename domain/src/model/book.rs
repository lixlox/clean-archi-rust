pub type BookId = String;
pub type Title = String;
pub type Description = String;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Book {
    pub id: BookId,
    pub title: Title,
    pub description: Description,
}
