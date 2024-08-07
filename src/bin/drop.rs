// Drop trait practice
struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1");
    }
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2");
    }
}

struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops");
    }
}
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo");
    }
}

fn main() {
    let _x = HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    };
    let foo = Foo;
    drop(foo);
    println!("Running");
}
