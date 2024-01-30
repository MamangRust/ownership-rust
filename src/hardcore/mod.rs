struct Book {
    title: String,
}

impl Book {
    fn get_title(&self) -> &String {
        &self.title
    }
}

pub fn main_hardcore() {
    let my_book = Book {
        title: String::from("Rust Programming"),
    };

    let title_reference;
    {
        let borrowed_book = my_book.get_title();
        title_reference = borrowed_book;
    }

    println!("Book Title: {}", title_reference);
}
