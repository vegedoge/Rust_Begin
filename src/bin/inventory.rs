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

#[derive(Debug)]
struct Inventory<T: Item> {
    items: Vec<T>,
}

impl <T: Item> Inventory<T> {
    fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    fn remove_item(&mut self, name: &str) {
        if let Some(index) = self.items.iter().position
            (|item| item.name() == name) {
                self.items.remove(index);
            }
    }

    fn find_item(&self, name: &str) -> Option<&T> {
        self.items.iter().find(|item| item.name() == name)
    }

}

fn main() {
    // Create book inventory
    let mut book_inventory = Inventory::new();
    let book = Book::new("Just for fun".to_string(), "Linus".to_string(), 262);
    book_inventory.add_item(book);
    if let Some(book) = book_inventory.find_item("Just for fun") {
        book.read();
    }

    // Create electronic inventory
    let mut e_inventory = Inventory::new();
    let drone = Electronic::new("Mavic2".to_string(), "DJI".to_string(), 2);
    let phone = Electronic::new("Pixel5".to_string(), "Google".to_string(), 1);
    e_inventory.add_item(drone);
    e_inventory.add_item(phone);
    if let Some(drone) = e_inventory.find_item("Mavic2") {
        drone.use_device();
    }
    
    println!("Book inventory:{:?}", book_inventory);
    println!("\nElectronic inventory:{:?}", e_inventory);

    e_inventory.remove_item("Mavic2");
    println!("\nElectronic inventory left:{:?}", e_inventory);
}