// Hashmap 
use std::collections::HashMap;

fn main(){
    let mut gems = HashMap::new();

    gems.insert("Red", 1);
    gems.insert("Blue", 5);
    gems.insert("Green", 3);

    // insert with old value
    let old = gems.insert("Blue", 10);
    println!("The poped out old value is {:?}", old);

    let new = gems.get("Blue");
    assert_eq!(new, Some(&10));
    println!("The new value of Blue is {:?}", new);

    // find and replace
    let new_green = gems.entry("Green").or_insert(15);
    println!("The new value for green is {}", *new_green);

    // get
    if let Some(blue_score) = gems.get(&"Blue"){
        println!("We have {blue_score} blue gems");
    }
    else{
        println!("We dont have Blue gems!")
    }
    
    // To hashmap
    let teams_list = vec![
        ("Chinese".to_string(), 100),
        ("American".to_string(), 131),
        ("Romanian".to_string(), 98),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
    // HashMap::from() is not implemeted for String
    println!("{:?}", teams_map);

    // text
    let text = "Hello world rust world";

    let mut text_map = HashMap::new(); 
    for word in text.split_whitespace(){
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", text_map);

}