pub struct Book {
    pub title: String,
    pub author: String,
    pub is_available: bool,
    pub borrower_name: String,
}

impl Book {
    pub fn book_create(
        title: String,
        author: String,
        is_available: bool,
        borrower_name: String,
    ) -> Self { // Self == Book
        Self {
            title,
            author,
            is_available,
            borrower_name,
        }
    }
}
