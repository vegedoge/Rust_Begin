// project inventory control by GPT
use::std::fmt::Debug;

// general trait Item with Debug implemented
trait Item: Debug {
    fn name(&self) -> &str;
}

#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    pages: u32,
}

impl Item for Book {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Book {
    fn new(name: String, author: String, pages: u32) -> Self {
        Book {name, author, pages}
    }

    fn read(&self) {
        println!("reading {}, authored by {}", self.name, self.author);
    }
}

#[derive(Debug)]
struct Electronic {
    name: String,
    brand: String,
    warranty_years: u32,
}

impl Item for Electronic {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Electronic {
    fn new(name: String, brand: String, warranty_years: u32) -> Self {
        Electronic{name, brand, warranty_years}
    }

    fn use_device(&self) {
        println!("using {}, made by {}", self.name, self.brand);
    }
}