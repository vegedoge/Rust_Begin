use std::{collections::HashMap, vec};

// practice for iterator
fn main()
{
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);

    let values = vec![1, 2, 3];
    // simulation of the impl of for iter
    let result = match IntoIterator::into_iter(values)
    {
        mut iter => loop
        {
            match iter.next()
            {
                Some(x) => println!("{}", x),
                None => break,
            }
        }
    };
    result;

    // difference between iter(), into_iter() and iter_mut()
    let test = vec![1, 2, 3];
    for v in test.into_iter()
    {
        println!("test: {}", v);
    }

    // println!("{:?}", test);

    let test = vec![1, 2, 3];
    for v in test.iter()
    {
        println!("test: {}", v);
    }

    println!("test after .iter() is {:?}", test);

    let mut test = vec![1, 2, 3];
    let mut test_iter_mut = test.iter_mut();
    if let Some(v) = test_iter_mut.next()
    {
        *v = 0;
    }
    println!("test after iter_mut() is {:?}", test);

    // collect
    let names = ["Asher", "Amir"];
    let ages = [28, 38];
    let folks: HashMap<_,_> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}", folks);

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));


}

struct Counter 
{
    count: u32,
}

impl Counter
{
    fn new() -> Counter 
    {
        Counter { count: 0}
    }
}

impl Iterator for Counter 
{
    type Item = u32;
    fn next(&mut self) -> Option::<Self::Item>
    {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
