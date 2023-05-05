// Ownership
// When we declare a variable inside a function, that variable is owned by that function
// We can move this variable (not copy) from one function to another transferring the ownership
// The function which currently, if ends, the variable will also get destroyed
// In order to share a variable among many functions, we use the concept of borrowing or referencing
// Referencing is done by using & keyword

struct Book {
  pages: i32,
  rating: i32
}

// Moving
fn display_page_count(book: Book){
  println!("Pages = {:?}", book.pages);
}

fn display_rating(book: Book){
  println!("Book rating = {:?}", book.rating);
}

// Borrowing
fn display_page_count2(book: &Book){
  println!("Pages = {:?}", book.pages);
}

fn display_rating2(book: &Book){
  println!("Book rating = {:?}", book.rating);
}

fn main() {
  let book = Book{pages: 100, rating: 4};
  display_page_count(book);

  // Below invocation doesn't work cause book is already destroyed once display_page_count fxn ends
  // As ownership of book is transferred from main() to display_page_count()
  // display_rating(book); 

  // Using borrowing technique
  let book = Book{pages: 200, rating: 3};
  display_page_count2(&book);
  display_rating2(&book);
}