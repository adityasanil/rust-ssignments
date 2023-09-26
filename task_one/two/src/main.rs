mod person;
mod book;
mod library;

use person::Person;
use book::Book;
use library::Library;

fn main() {
    let p: Person = Person::person_create("Harry".to_string(), 23);
    p.person_details();

    let first_book: Book = Book::book_create("Theory of Everything".to_string(), "Stephen Hawking".to_string(), true, "".to_string());
    let second_book: Book = Book::book_create("The 48 Laws Of Power".to_string(), "Robert Greene".to_string(), true, "".to_string());

    let mut library: Library = Library::new();
    library.add_book(first_book);
    library.add_book(second_book);

    library.get_all_books();

    library.checkout_book("Theory of Everything".to_string(), &p.name);

    library.get_available_books();
    library.get_borrowed_books();
    library.return_book("Theory of Everything".to_string());

    library.get_available_books();
}
