use crate::book::Book;

pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            books: Vec::new()
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn get_all_books(&self) {
        println!("\nList of all books in the library");
        self.books.iter().for_each(|book| println!("Book name: {} by {}", book.title, book.author))
    }

    pub fn checkout_book(&mut self, title: String, borrower_name: &String) {
        for book in self.books.iter_mut() {
            if book.title == title {
                if book.is_available {
                    book.is_available = false;
                    book.borrower_name = String::from(borrower_name);
                } else {
                    println!("\nBook unavailable");
                }
            } 
        }
    }

    pub fn return_book(&mut self, title: String) {
        for book in self.books.iter_mut() {
            if book.title == title {
                if !book.is_available {
                    println!("\nBook {} returned by {}", book.title, book.borrower_name);
                    book.is_available = true;
                    book.borrower_name = String::from("");
                } else {
                    println!("\nUnable to return an available book");
                }
            } 
        }
    }

    pub fn get_borrowed_books(&self) {
        println!("\nList of all borrowed books");
        self.books.iter().filter(|book| !book.is_available).for_each(|book| println!("Book: {} borrowed by {}", book.title, book.borrower_name))
    }

    pub fn get_available_books(&self) {
        println!("\nList of all available books");
        self.books.iter().filter(|book| book.is_available).for_each(|book| println!("Book name: {} by {}", book.title, book.author))
    }

}
